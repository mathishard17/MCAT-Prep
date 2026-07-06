# Copyright: Ankitects Pty Ltd and contributors
# License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

from __future__ import annotations

import html
from copy import deepcopy
from dataclasses import dataclass
from typing import Any

import aqt
import aqt.operations
from anki.collection import Collection, OpChanges
from anki.decks import DeckCollapseScope, DeckId, DeckTreeNode
from aqt import AnkiQt, gui_hooks, mcat_ui
from aqt.deckoptions import display_options_for_deck_id
from aqt.operations import QueryOp
from aqt.operations.deck import (
    add_deck_dialog,
    remove_decks,
    rename_deck,
    reparent_decks,
    set_current_deck,
    set_deck_collapsed,
)
from aqt.qt import *
from aqt.sound import av_player
from aqt.theme import Theme
from aqt.toolbar import BottomBar
from aqt.utils import getOnlyText, openLink, shortcut, showInfo, tooltip, tr


class DeckBrowserBottomBar:
    def __init__(self, deck_browser: DeckBrowser) -> None:
        self.deck_browser = deck_browser


@dataclass
class RenderData:
    """Data from collection that is required to show the page."""

    tree: DeckTreeNode
    current_deck_id: DeckId
    studied_today: str
    sched_upgrade_required: bool
    # MCAT Readiness Dashboard (empty string when there is no MCAT deck).
    mcat_dashboard: str = ""
    mcat_deck_id: DeckId | None = None


@dataclass
class DeckBrowserContent:
    """Stores sections of HTML content that the deck browser will be
    populated with.

    Attributes:
        tree {str} -- HTML of the deck tree section
        stats {str} -- HTML of the stats section
    """

    tree: str
    stats: str


@dataclass
class RenderDeckNodeContext:
    current_deck_id: DeckId


class DeckBrowser:
    _render_data: RenderData

    def __init__(self, mw: AnkiQt) -> None:
        self.mw = mw
        self.web = mw.web
        self.bottom = BottomBar(mw, mw.bottomWeb)
        self.scrollPos = QPoint(0, 0)
        self._refresh_needed = False
        self._mcat_demo_import_attempted = False

    def show(self) -> None:
        av_player.stop_and_clear_queue()
        self.web.set_bridge_command(self._linkHandler, self)
        # redraw top bar for theme change
        self.mw.toolbar.redraw()
        self._ensure_mcat_demo_imported()
        self.refresh()

    def _ensure_mcat_demo_imported(self) -> None:
        if self._mcat_demo_import_attempted:
            return

        self._mcat_demo_import_attempted = True

        def import_mcat_demo_deck(col: Collection) -> bool:
            # Production install: only the full "MCAT" library deck (no "MCAT Demo").
            deck_already_exists = col.decks.id_for_name("MCAT") is not None
            col._backend.import_mcat_demo_deck()
            # Turn FSRS on by default the first time the MCAT deck is set up. Guarded
            # so it runs once and a later manual toggle-off in deck options sticks.
            if not col.get_config("mcatFsrsDefaulted", False):
                col.set_config("fsrs", True)
                col.set_config("mcatFsrsDefaulted", True)
            return not deck_already_exists

        def on_mcat_demo_imported(deck_created: bool) -> None:
            self.refresh()
            if deck_created:
                showInfo("Your MCAT deck has been installed locally.")

        QueryOp(
            parent=self.mw,
            op=import_mcat_demo_deck,
            success=on_mcat_demo_imported,
        ).run_in_background()

    def refresh(self) -> None:
        self._renderPage()
        self._refresh_needed = False

    def refresh_if_needed(self) -> None:
        if self._refresh_needed:
            self.refresh()

    def op_executed(
        self, changes: OpChanges, handler: object | None, focused: bool
    ) -> bool:
        if changes.study_queues and handler is not self:
            self._refresh_needed = True

        if focused:
            self.refresh_if_needed()

        return self._refresh_needed

    # Event handlers
    ##########################################################################

    def _linkHandler(self, url: str) -> Any:
        if ":" in url:
            (cmd, arg) = url.split(":", 1)
        else:
            cmd = url
            arg = ""
        if cmd == "open":
            self.set_current_deck(DeckId(int(arg)))
        elif cmd == "opts":
            self._showOptions(arg)
        elif cmd == "shared":
            self._onShared()
        elif cmd == "import":
            self.mw.onImport()
        elif cmd == "create":
            self._on_create()
        elif cmd == "drag":
            source, target = arg.split(",")
            self._handle_drag_and_drop(DeckId(int(source)), DeckId(int(target or 0)))
        elif cmd == "collapse":
            self._collapse(DeckId(int(arg)))
        elif cmd == "v2upgrade":
            self._confirm_upgrade()
        elif cmd == "v2upgradeinfo":
            if self.mw.col.sched_ver() == 1:
                openLink("https://faqs.ankiweb.net/the-anki-2.1-scheduler.html")
            else:
                openLink("https://faqs.ankiweb.net/the-2021-scheduler.html")
        elif cmd == "select":
            set_current_deck(
                parent=self.mw, deck_id=DeckId(int(arg))
            ).run_in_background()
        elif cmd == "theme":
            self._toggle_theme()
        elif cmd == "mcatStudy":
            self._open_mcat_deck()
        elif cmd == "mcatPlanSave":
            self._set_mcat_plan(arg)
        elif cmd == "mcatStart":
            self._start_mcat_topic(arg)
        elif cmd == "mcatSection":
            self._select_mcat_section(arg)
        elif cmd == "mcatSectionClear":
            self._select_mcat_section("")
        return False

    def _toggle_theme(self) -> None:
        new_theme = Theme.DARK if self.mw.pm.theme() == Theme.LIGHT else Theme.LIGHT
        self.mw.set_theme(new_theme)
        self.refresh()

    def set_current_deck(self, deck_id: DeckId) -> None:
        set_current_deck(parent=self.mw, deck_id=deck_id).success(
            lambda _: self.mw.onOverview()
        ).run_in_background(initiator=self)

    # HTML generation
    ##########################################################################

    _body = """
<center>
<div class="home-theme-toggle">
<button onclick="pycmd('theme');">%(theme_toggle)s</button>
</div>
%(mcat)s
%(deck_head)s
<table cellspacing=0 cellpadding=3>
%(tree)s
</table>

<br>
%(stats)s
</center>
"""

    def _renderPage(self, reuse: bool = False) -> None:
        if not reuse:

            def get_data(col: Collection) -> RenderData:
                mcat_did = self._mcat_deck_id(col)
                return RenderData(
                    tree=col.sched.deck_due_tree(),
                    current_deck_id=col.decks.get_current_id(),
                    studied_today=col.studied_today(),
                    sched_upgrade_required=not col.v3_scheduler(),
                    mcat_dashboard=self._mcat_dashboard_html(col, mcat_did),
                    mcat_deck_id=mcat_did,
                )

            def success(output: RenderData) -> None:
                self._render_data = output
                self.__renderPage(None)

            QueryOp(
                parent=self.mw,
                op=get_data,
                success=success,
            ).run_in_background()
        else:
            self.web.evalWithCallback("window.pageYOffset", self.__renderPage)

    def __renderPage(self, offset: int | None) -> None:
        data = self._render_data
        content = DeckBrowserContent(
            tree=self._renderDeckTree(data.tree),
            stats=self._renderStats(),
        )
        theme_label = "Dark mode" if self.mw.pm.theme() == Theme.LIGHT else "Light mode"
        gui_hooks.deck_browser_will_render_content(self, content)
        deck_head = (
            '<div class="mcat-decks-title">All decks</div>'
            if data.mcat_dashboard
            else ""
        )
        self.web.stdHtml(
            self._v1_upgrade_message(data.sched_upgrade_required)
            + self._body
            % (
                content.__dict__
                | dict(
                    theme_toggle=theme_label,
                    mcat=data.mcat_dashboard,
                    deck_head=deck_head,
                )
            ),
            css=["css/deckbrowser.css"],
            js=[
                "js/vendor/jquery.min.js",
                "js/vendor/jquery-ui.min.js",
                "js/deckbrowser.js",
            ],
            context=self,
        )
        self._drawButtons()
        if offset is not None:
            self._scrollToOffset(offset)
        gui_hooks.deck_browser_did_render(self)

    def _scrollToOffset(self, offset: int) -> None:
        self.web.eval("window.scrollTo(0, %d, 'instant');" % offset)

    def _renderStats(self) -> str:
        return '<div id="studiedToday"><span>{}</span></div>'.format(
            self._render_data.studied_today
        )

    # MCAT Readiness Dashboard
    ##########################################################################

    def _mcat_deck_id(self, col: Collection) -> DeckId | None:
        """The primary MCAT library deck, if this collection has one."""
        try:
            return col.decks.id_for_name("MCAT")
        except Exception:
            return None

    def _mcat_dashboard_html(self, col: Collection, deck_id: DeckId | None) -> str:
        """Build the readiness dashboard for the home screen, or "" when there is
        no MCAT deck / concept data to show. Runs off the UI thread."""
        if deck_id is None:
            return ""
        try:
            status = col._backend.get_concept_scheduler_status(deck_id)
        except Exception:
            return ""
        payload = self._mcat_status_dict(status)
        if not payload["nodes"]:
            return ""
        return mcat_ui.build_dashboard_html(payload)

    @staticmethod
    def _mcat_status_dict(status: Any) -> dict[str, Any]:
        """Convert the concept scheduler status proto into the plain dict the
        dashboard consumes. Only reads existing fields; never mutates state."""

        def fringe(value: int) -> str:
            return {1: "inner", 2: "outer"}.get(value, "locked")

        def section(value: int) -> str:
            return {
                0: "Bio/Biochem",
                1: "Chem/Phys",
                2: "Psych/Soc",
                3: "CARS",
            }.get(value, "Unknown")

        nodes = [
            dict(
                id=node.id,
                mastery=node.mastery,
                fringe=fringe(node.fringe),
                answered=node.answered,
                positive=node.positive,
                negative=node.negative,
                memory=node.memory,
                recommended=node.recommended,
            )
            for node in status.graph.nodes
        ]
        evidence = status.evidence
        has_selected_section = status.HasField("selected_section")
        return dict(
            nodes=nodes,
            hasSelectedSection=has_selected_section,
            selectedSection=(
                section(status.selected_section) if has_selected_section else ""
            ),
            edges=[
                dict(prerequisiteId=edge.prerequisite_id, targetId=edge.target_id)
                for edge in status.graph.edges
            ],
            sectionScores=[
                dict(
                    section=section(score.section),
                    enoughEvidence=score.enough_evidence,
                    coverage=score.coverage,
                    readinessCenter=score.readiness_center,
                    readinessLower=score.readiness_lower,
                    readinessUpper=score.readiness_upper,
                    readinessStandardError=score.readiness_standard_error,
                    performanceCenter=score.performance_center,
                    performanceLower=score.performance_lower,
                    performanceUpper=score.performance_upper,
                    performanceStandardError=score.performance_standard_error,
                    theta=score.theta,
                    sectionMastery=score.section_mastery,
                    answeredItems=score.answered_items,
                    requiredItems=score.required_items,
                    sectionMemory=score.section_memory,
                    sectionHasMemory=score.section_has_memory,
                )
                for score in status.section_scores
            ],
            # Overall evidence gate (drives the abstain / "building" states).
            evidenceEnough=evidence.kind == 1,
            evidenceSeenCards=evidence.seen_cards,
            evidenceRequiredCards=evidence.required_seen_cards,
            overallMemory=status.overall_memory,
            hasMemory=status.has_memory,
            hasProjection=status.has_projection,
            projectedTotal=status.projected_total,
            projectedTotalLower=status.projected_total_lower,
            projectedTotalUpper=status.projected_total_upper,
            examTimestamp=status.exam_timestamp,
            hasTarget=status.has_target,
            targetTotalScore=status.target_total_score,
            probabilityHitTarget=status.probability_hit_target,
        )

    def _open_mcat_deck(self) -> None:
        did = getattr(self._render_data, "mcat_deck_id", None)
        if did is not None:
            self.set_current_deck(did)

    def _set_mcat_plan(self, raw: str) -> None:
        """Persist the learner's exam date / target from the dashboard editor.
        ``raw`` is "<epoch>,<target>"; 0 clears that field."""
        did = getattr(self._render_data, "mcat_deck_id", None)
        if did is None:
            return
        try:
            epoch_str, target_str = raw.split(",", 1)
            epoch = int(epoch_str)
            target = int(target_str)
        except ValueError:
            return
        from anki.scheduler_pb2 import SetConceptExamSettingsRequest

        request = SetConceptExamSettingsRequest(deck_id=did)
        if epoch > 0:
            request.exam_timestamp = epoch
        if 472 <= target <= 528:
            request.target_total_score = target
        try:
            self.mw.col._backend.set_concept_exam_settings(request)
        except Exception as exc:
            showInfo(f"Couldn't save your study plan: {exc}")
            return
        tooltip("Study plan updated", period=2500)
        self.refresh()

    def _start_mcat_topic(self, topic: str) -> None:
        did = getattr(self._render_data, "mcat_deck_id", None)
        topic = topic.strip()
        if did is None or not topic:
            return
        from anki.scheduler_pb2 import SetConceptSelectedTopicRequest

        try:
            self.mw.col._backend.set_concept_selected_topic(
                SetConceptSelectedTopicRequest(deck_id=did, topic=topic)
            )
        except Exception as exc:
            tooltip(f"Couldn't start that topic: {exc}")
            return
        short = topic.split("::")[-1].replace("_", " ")
        tooltip(f"Next up: <b>{short}</b>", period=3000)
        self.refresh()

    # Section labels indexed by the McatSection enum value (0-3), used for the
    # "study by section" tooltips and to validate the incoming index.
    _MCAT_SECTION_LABELS = ["Bio/Biochem", "Chem/Phys", "Psych/Soc", "CARS"]

    def _select_mcat_section(self, raw: str) -> None:
        """Restrict studying to one MCAT section (``raw`` is the section index
        0-3), or clear the restriction when ``raw`` is empty. Calls the backend
        ``set_concept_selected_section`` RPC and refreshes the dashboard."""
        did = getattr(self._render_data, "mcat_deck_id", None)
        if did is None:
            return
        from anki.scheduler_pb2 import SetConceptSelectedSectionRequest

        raw = raw.strip()
        request = SetConceptSelectedSectionRequest(deck_id=did)
        label = "all sections"
        if raw != "":
            try:
                index = int(raw)
            except ValueError:
                return
            if not 0 <= index < len(self._MCAT_SECTION_LABELS):
                return
            request.section = index
            label = self._MCAT_SECTION_LABELS[index]
        try:
            self.mw.col._backend.set_concept_selected_section(request)
        except Exception as exc:
            tooltip(f"Couldn't switch section: {exc}")
            return
        if raw == "":
            tooltip("Studying <b>all sections</b>", period=3000)
        else:
            tooltip(f"Studying <b>{label}</b>", period=3000)
        self.refresh()

    def _renderDeckTree(self, top: DeckTreeNode) -> str:
        buf = """
<tr><th colspan=5 align=start>{}</th>
<th class=count>{}</th>
<th class=count>{}</th>
<th class=count>{}</th>
<th class=optscol></th></tr>""".format(
            tr.decks_deck(),
            tr.actions_new(),
            tr.decks_learn_header(),
            tr.decks_review_header(),
        )
        buf += self._topLevelDragRow()

        ctx = RenderDeckNodeContext(current_deck_id=self._render_data.current_deck_id)

        for child in top.children:
            buf += self._render_deck_node(child, ctx)

        return buf

    def _render_deck_node(self, node: DeckTreeNode, ctx: RenderDeckNodeContext) -> str:
        if node.collapsed:
            prefix = "+"
        else:
            prefix = "−"

        def indent() -> str:
            return "&nbsp;" * 6 * (node.level - 1)

        if node.deck_id == ctx.current_deck_id:
            klass = "deck current"
        else:
            klass = "deck"

        buf = (
            "<tr class='%s' id='%d' onclick='if(event.shiftKey) return pycmd(\"select:%d\")'>"
            % (
                klass,
                node.deck_id,
                node.deck_id,
            )
        )
        # deck link
        if node.children:
            collapse = (
                "<a class=collapse href=# onclick='return pycmd(\"collapse:%d\")'>%s</a>"
                % (node.deck_id, prefix)
            )
        else:
            collapse = "<span class=collapse></span>"
        if node.filtered:
            extraclass = "filtered"
        else:
            extraclass = ""
        buf += """

        <td class=decktd colspan=5>%s%s<a class="deck %s"
        href=# onclick="return pycmd('open:%d')">%s</a></td>""" % (
            indent(),
            collapse,
            extraclass,
            node.deck_id,
            html.escape(node.name),
        )

        # due counts
        def nonzeroColour(cnt: int, klass: str) -> str:
            if not cnt:
                klass = "zero-count"
            return f'<span class="{klass}">{cnt}</span>'

        review = nonzeroColour(node.review_count, "review-count")
        learn = nonzeroColour(node.learn_count, "learn-count")

        buf += ("<td align=end>%s</td>" * 3) % (
            nonzeroColour(node.new_count, "new-count"),
            learn,
            review,
        )
        # options
        buf += (
            "<td align=center class=opts><a onclick='return pycmd(\"opts:%d\");'>"
            "<img src='/_anki/imgs/gears.svg' class=gears></a></td></tr>" % node.deck_id
        )
        # children
        if not node.collapsed:
            for child in node.children:
                buf += self._render_deck_node(child, ctx)
        return buf

    def _topLevelDragRow(self) -> str:
        return "<tr class='top-level-drag-row'><td colspan='6'>&nbsp;</td></tr>"

    # Options
    ##########################################################################

    def _showOptions(self, did: str) -> None:
        m = QMenu(self.mw)
        a = m.addAction(tr.actions_rename())
        assert a is not None
        qconnect(a.triggered, lambda b, did=did: self._rename(DeckId(int(did))))
        a = m.addAction(tr.actions_options())
        assert a is not None
        qconnect(a.triggered, lambda b, did=did: self._options(DeckId(int(did))))
        a = m.addAction(tr.actions_export())
        assert a is not None
        qconnect(a.triggered, lambda b, did=did: self._export(DeckId(int(did))))
        a = m.addAction(tr.actions_delete())
        assert a is not None
        qconnect(a.triggered, lambda b, did=did: self._delete(DeckId(int(did))))
        gui_hooks.deck_browser_will_show_options_menu(m, int(did))
        m.popup(QCursor.pos())

    def _export(self, did: DeckId) -> None:
        self.mw.onExport(did=did)

    def _rename(self, did: DeckId) -> None:
        def prompt(name: str) -> None:
            new_name = getOnlyText(
                tr.decks_new_deck_name(), default=name, title=tr.actions_rename()
            )
            if not new_name or new_name == name:
                return
            else:
                rename_deck(
                    parent=self.mw, deck_id=did, new_name=new_name
                ).run_in_background()

        QueryOp(
            parent=self.mw, op=lambda col: col.decks.name(did), success=prompt
        ).run_in_background()

    def _options(self, did: DeckId) -> None:
        display_options_for_deck_id(did)

    def _collapse(self, did: DeckId) -> None:
        node = self.mw.col.decks.find_deck_in_tree(self._render_data.tree, did)
        if node:
            node.collapsed = not node.collapsed
            set_deck_collapsed(
                parent=self.mw,
                deck_id=did,
                collapsed=node.collapsed,
                scope=DeckCollapseScope.REVIEWER,
            ).run_in_background()
            self._renderPage(reuse=True)

    def _handle_drag_and_drop(self, source: DeckId, target: DeckId) -> None:
        reparent_decks(
            parent=self.mw, deck_ids=[source], new_parent=target
        ).run_in_background()

    def _delete(self, did: DeckId) -> None:
        deck = self.mw.col.decks.find_deck_in_tree(self._render_data.tree, did)
        assert deck is not None
        deck_name = deck.name
        remove_decks(
            parent=self.mw, deck_ids=[did], deck_name=deck_name
        ).run_in_background()

    # Top buttons
    ######################################################################

    drawLinks = [
        ["", "shared", tr.decks_get_shared()],
        ["", "create", tr.decks_create_deck()],
        ["Ctrl+Shift+I", "import", tr.decks_import_file()],
    ]

    def _drawButtons(self) -> None:
        buf = ""
        drawLinks = deepcopy(self.drawLinks)
        for b in drawLinks:
            if b[0]:
                b[0] = tr.actions_shortcut_key(val=shortcut(b[0]))
            buf += """
<button title='%s' onclick='pycmd(\"%s\");'>%s</button>""" % tuple(b)
        self.bottom.draw(
            buf=buf,
            link_handler=self._linkHandler,
            web_context=DeckBrowserBottomBar(self),
        )

    def _onShared(self) -> None:
        openLink(f"{aqt.appShared}decks/")

    def _on_create(self) -> None:
        if op := add_deck_dialog(
            parent=self.mw, default_text=self.mw.col.decks.current()["name"]
        ):
            op.run_in_background()

    ######################################################################

    def _v1_upgrade_message(self, required: bool) -> str:
        if not required:
            return ""

        update_required = tr.scheduling_update_required().replace("V2", "v3")

        return f"""
<center>
<div class=callout>
    <div>
      {update_required}
    </div>
    <div>
      <button onclick='pycmd("v2upgrade")'>
        {tr.scheduling_update_button()}
      </button>
      <button onclick='pycmd("v2upgradeinfo")'>
        {tr.scheduling_update_more_info_button()}
      </button>
    </div>
</div>
</center>
"""

    def _confirm_upgrade(self) -> None:
        if self.mw.col.sched_ver() == 1:
            self.mw.col.mod_schema(check=True)
            self.mw.col.upgrade_to_v2_scheduler()
        self.mw.col.set_v3_scheduler(True)

        showInfo(tr.scheduling_update_done())
        self.refresh()
