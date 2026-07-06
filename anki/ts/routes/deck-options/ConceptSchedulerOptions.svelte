<!--
Copyright: Ankitects Pty Ltd and contributors
License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html
-->
<script lang="ts">
    import {
        getConceptSchedulerStatus,
        setConceptSelectedTopic,
    } from "@generated/backend";
    import * as tr from "@generated/ftl";
    import { onMount } from "svelte";
    import { cubicOut } from "svelte/easing";
    import { tweened } from "svelte/motion";

    import DynamicallySlottable from "$lib/components/DynamicallySlottable.svelte";
    import Item from "$lib/components/Item.svelte";
    import SettingTitle from "$lib/components/SettingTitle.svelte";
    import SwitchRow from "$lib/components/SwitchRow.svelte";
    import TitledContainer from "$lib/components/TitledContainer.svelte";

    import type { DeckOptionsState } from "./lib";

    export let state: DeckOptionsState;
    export let api: Record<string, never>;

    const limits = state.deckLimits;
    type ConceptStatus = Awaited<ReturnType<typeof getConceptSchedulerStatus>>;
    type ConceptNode = NonNullable<ConceptStatus["graph"]>["nodes"][number];
    type ConceptSectionScore = ConceptStatus["sectionScores"][number];
    type DisplayEdge = { prerequisiteId: string; targetId: string };
    type FringeBucket = "inner" | "outer" | "locked";
    type Units = {
        pos: Record<string, { col: number; row: number }>;
        cols: number;
        rows: number;
    };

    let status: ConceptStatus | null = null;
    let liveNodes: ConceptNode[] = [];

    $: liveNodes = status?.graph?.nodes ?? [];

    onMount(() => {
        void refreshStatus();
    });

    $: if ($limits.conceptSchedulerEnabled) {
        void refreshStatus();
    }

    async function refreshStatus(): Promise<void> {
        if (!$limits.conceptSchedulerEnabled) {
            status = null;
            return;
        }

        status = await getConceptSchedulerStatus({
            deckId: state.getTargetDeckId(),
        });
    }

    function enumNumber(value: unknown): number | null {
        if (typeof value === "number") {
            return value;
        }

        if (typeof value === "string" && /^\d+$/.test(value)) {
            return Number(value);
        }

        return null;
    }

    function enumText(value: unknown): string {
        if (value === null || value === undefined) {
            return "";
        }

        if (typeof value === "string") {
            return value;
        }

        if (typeof value === "number" || typeof value === "boolean") {
            return String(value);
        }

        return JSON.stringify(value);
    }

    function fringeFor(node: ConceptNode): FringeBucket {
        const numericFringe = enumNumber(node.fringe);
        if (numericFringe === 1) {
            return "inner";
        }
        if (numericFringe === 2) {
            return "outer";
        }

        const fringe = enumText(node.fringe).toLowerCase();
        if (fringe.includes("inner")) {
            return "inner";
        }
        if (fringe.includes("outer")) {
            return "outer";
        }

        return "locked";
    }

    function percent(value: number): string {
        return `${Math.round(value * 100)}%`;
    }

    function evidencePercent(status: ConceptStatus): string {
        const evidence = status.evidence;
        if (!evidence || evidence.requiredSeenCards === 0) {
            return "0%";
        }

        return percent(Math.min(1, evidence.seenCards / evidence.requiredSeenCards));
    }

    function evidenceKind(status: ConceptStatus): string {
        const kind = status.evidence?.kind;
        if (kind === undefined || kind === null) {
            return "unknown";
        }

        const label = String(kind).toLowerCase();
        if (label.includes("enough") || kind === 1) {
            return "enough";
        }
        if (label.includes("insufficient") || kind === 0) {
            return "insufficient";
        }

        return String(kind);
    }

    function topicLabel(topic: string | null | undefined): string {
        return topic || "none";
    }

    function numberField(node: ConceptNode, ...keys: string[]): number {
        const record = node as unknown as Record<string, unknown>;

        for (const key of keys) {
            const value = record[key];
            if (typeof value === "number") {
                return value;
            }
        }

        return 0;
    }

    function answeredCount(node: ConceptNode): number {
        return numberField(node, "answered", "answeredCount", "answered_count");
    }

    function masteryScore(node: ConceptNode): number {
        return numberField(node, "mastery");
    }

    function totalGraphAnswers(): number {
        return (
            status?.graph?.nodes.reduce(
                (total, node) => total + answeredCount(node),
                0,
            ) ?? 0
        );
    }

    function sectionName(section: unknown): string {
        const numericSection = enumNumber(section);
        if (numericSection === 0) {
            return "Bio/Biochem";
        }
        if (numericSection === 1) {
            return "Chem/Phys";
        }
        if (numericSection === 2) {
            return "Psych/Soc";
        }
        if (numericSection === 3) {
            return "CARS";
        }

        const text = enumText(section).toLowerCase();
        if (text.includes("bio")) {
            return "Bio/Biochem";
        }
        if (text.includes("chem")) {
            return "Chem/Phys";
        }
        if (text.includes("psych")) {
            return "Psych/Soc";
        }
        if (text.includes("cars")) {
            return "CARS";
        }

        return enumText(section) || "Unknown section";
    }

    function scoreRange(
        score: ConceptSectionScore,
        prefix: "performance" | "readiness",
    ): string {
        const record = score as unknown as Record<string, unknown>;
        const lower = Number(record[`${prefix}Lower`] ?? 0);
        const upper = Number(record[`${prefix}Upper`] ?? 0);

        return `${Math.round(lower)}-${Math.round(upper)}`;
    }

    // -- Concept map (shared "linear paths" layout with the reviewer sidebar) ---

    // MCAT super-section colours: Bio/Biochem blue, Chem/Phys red, Psych/Soc
    // green, CARS amber. Discipline lanes stack these into clean chains.
    const DISCIPLINE_ORDER = [
        "Bio",
        "Biochem",
        "GenChem",
        "Orgo",
        "Physics",
        "PsychSoc",
        "CARS",
    ];
    const SECTION_COLORS: Record<string, string> = {
        Bio_Biochem: "#4f74d6",
        Chem_Phys: "#d65f5f",
        Psych_Soc: "#5aa469",
        CARS: "#c99a3a",
        Other: "#8a8a8a",
    };

    function disciplineOf(id: string): string {
        return id.split("::")[0] || "Other";
    }

    function sectionKey(id: string): string {
        const d = disciplineOf(id);
        if (d === "Bio" || d === "Biochem") {
            return "Bio_Biochem";
        }
        if (d === "GenChem" || d === "Orgo" || d === "Physics") {
            return "Chem_Phys";
        }
        if (d === "PsychSoc") {
            return "Psych_Soc";
        }
        if (d === "CARS") {
            return "CARS";
        }
        return "Other";
    }

    function sectionColor(id: string): string {
        return SECTION_COLORS[sectionKey(id)] ?? SECTION_COLORS.Other;
    }

    function shortLabel(id: string): string {
        const parts = id.split("::");
        return parts[parts.length - 1].replaceAll("_", " ");
    }

    // Layered layout: topological longest-path depth = column, discipline = lane.
    function layeredUnits(ids: string[], edges: DisplayEdge[]): Units {
        const idset = new Set(ids);
        const succ = new Map<string, string[]>(ids.map((id) => [id, []]));
        const indeg = new Map<string, number>(ids.map((id) => [id, 0]));
        for (const edge of edges) {
            if (idset.has(edge.prerequisiteId) && idset.has(edge.targetId)) {
                succ.get(edge.prerequisiteId)!.push(edge.targetId);
                indeg.set(edge.targetId, (indeg.get(edge.targetId) ?? 0) + 1);
            }
        }
        const depth = new Map<string, number>(ids.map((id) => [id, 0]));
        const remaining = new Map(indeg);
        const queue = ids.filter((id) => (remaining.get(id) ?? 0) === 0);
        while (queue.length) {
            const u = queue.shift()!;
            for (const v of succ.get(u) ?? []) {
                if ((depth.get(v) ?? 0) < (depth.get(u) ?? 0) + 1) {
                    depth.set(v, (depth.get(u) ?? 0) + 1);
                }
                remaining.set(v, (remaining.get(v) ?? 0) - 1);
                if ((remaining.get(v) ?? 0) === 0) {
                    queue.push(v);
                }
            }
        }
        const laneOf = (id: string) => {
            const d = disciplineOf(id);
            return DISCIPLINE_ORDER.includes(d) ? d : "Other";
        };
        const lanes = [...DISCIPLINE_ORDER, "Other"];
        const pos: Record<string, { col: number; row: number }> = {};
        let rowTop = 0;
        let maxCol = 0;
        for (const lane of lanes) {
            const members = ids.filter((id) => laneOf(id) === lane);
            if (!members.length) {
                continue;
            }
            members.sort(
                (a, b) =>
                    (depth.get(a) ?? 0) - (depth.get(b) ?? 0) || (a < b ? -1 : 1),
            );
            const colCount = new Map<number, number>();
            let laneRows = 0;
            for (const id of members) {
                const c = depth.get(id) ?? 0;
                const r = colCount.get(c) ?? 0;
                colCount.set(c, r + 1);
                pos[id] = { col: c, row: rowTop + r };
                if (r + 1 > laneRows) {
                    laneRows = r + 1;
                }
                if (c > maxCol) {
                    maxCol = c;
                }
            }
            rowTop += laneRows + 1;
        }
        return { pos, cols: maxCol + 1, rows: Math.max(rowTop, 1) };
    }

    function computeDeps(edges: DisplayEdge[]): Record<string, number> {
        const counts: Record<string, number> = {};
        for (const edge of edges) {
            counts[edge.prerequisiteId] = (counts[edge.prerequisiteId] ?? 0) + 1;
        }
        return counts;
    }

    const COLW = 150;
    const ROWH = 42;
    const PAD = 26;
    const LABELPAD = 120;

    let graphEl: HTMLDivElement | undefined;
    let hoverId: string | null = null;
    let didFit = false;
    let zoomAnchor:
        | { cx: number; cy: number; ax: number; ay: number; z0: number }
        | null = null;

    const zoom = tweened(1, { duration: 220, easing: cubicOut });
    let zoomVal = 1;
    $: zoomVal = $zoom;

    // Read everything from the live DTO — no fabricated nodes / positions.
    $: allIds = liveNodes.map((node) => node.id);
    $: liveEdges = (status?.graph?.edges ?? []).map((edge) => ({
        prerequisiteId: edge.prerequisiteId,
        targetId: edge.targetId,
    }));
    $: units = layeredUnits(allIds, liveEdges);
    $: deps = computeDeps(liveEdges);
    $: maxDep = Math.max(1, ...Object.values(deps));
    $: currentTopic =
        status?.session?.selectedTopic || status?.session?.activeTopic || null;
    // Spotlight (edge/neighbour dimming) follows the hovered node only, so the
    // full map stays fully visible by default; the current topic keeps its ring.
    $: activeId = hoverId;

    $: nodeViews = liveNodes
        .filter((node) => units.pos[node.id])
        .map((node) => {
            const u = units.pos[node.id];
            const bucket = fringeFor(node);
            const dep = deps[node.id] ?? 0;
            const parts = node.id.split("::");
            return {
                id: node.id,
                col: u.col,
                row: u.row,
                sec: sectionColor(node.id),
                dotRem: 0.5 + 0.8 * Math.sqrt(dep / maxDep),
                mastery: Math.max(0, Math.min(1, masteryScore(node))),
                bucket,
                startable: bucket === "outer",
                recommended: !!node.recommended,
                current: node.id === currentTopic,
                label: shortLabel(node.id),
                subject: disciplineOf(node.id),
                name: parts[parts.length - 1].replaceAll("_", " "),
            };
        });

    $: shownEdges = liveEdges.filter(
        (edge) => units.pos[edge.prerequisiteId] && units.pos[edge.targetId],
    );
    $: edgeViews = shownEdges.map((edge) => ({
        from: edge.prerequisiteId,
        to: edge.targetId,
        d: edgePathD(edge, zoomVal, zoomVal >= 0.6),
    }));
    $: neighbourSet = buildNeighbours(shownEdges);
    $: readyCount = nodeViews.filter((node) => node.startable).length;
    $: showLabels = zoomVal >= (nodeViews.length > 16 ? 1.05 : 0.85);
    $: canvasW = PAD * 2 + Math.max(units.cols - 1, 0) * COLW * zoomVal + LABELPAD;
    $: canvasH = PAD * 2 + Math.max(units.rows - 1, 0) * ROWH * zoomVal + ROWH;

    // Fit the whole map once it first has nodes + a measured container.
    $: if (graphEl && liveNodes.length && units.cols > 0 && !didFit) {
        didFit = true;
        requestAnimationFrame(() => fit(false));
    }

    $: reflowScroll(zoomVal);

    function nx(col: number, z: number): number {
        return PAD + col * COLW * z;
    }

    function ny(row: number, z: number): number {
        return PAD + row * ROWH * z;
    }

    function dotRadiusPx(id: string): number {
        const dep = deps[id] ?? 0;
        return (0.5 + 0.8 * Math.sqrt(dep / maxDep)) * 8;
    }

    // prerequisite -> target: a gentle curve shortened to stop just outside each
    // dot, plus a small stroked arrowhead so the direction is unmistakable.
    function edgePathD(edge: DisplayEdge, z: number, arrows: boolean): string {
        const a = units.pos[edge.prerequisiteId];
        const b = units.pos[edge.targetId];
        if (!a || !b) {
            return "";
        }
        const sx = nx(a.col, z);
        const sy = ny(a.row, z);
        const ex = nx(b.col, z);
        const ey = ny(b.row, z);
        const dx = ex - sx;
        const dy = ey - sy;
        const len = Math.hypot(dx, dy) || 1;
        const ux = dx / len;
        const uy = dy / len;
        const sGap = dotRadiusPx(edge.prerequisiteId) + 2;
        const eGap = dotRadiusPx(edge.targetId) + 5;
        const sx2 = sx + ux * sGap;
        const sy2 = sy + uy * sGap;
        const ex2 = ex - ux * eGap;
        const ey2 = ey - uy * eGap;
        const bow = Math.min(16, len * 0.1);
        const mx = (sx2 + ex2) / 2 - uy * bow;
        const my = (sy2 + ey2) / 2 + ux * bow;
        let d = `M ${sx2.toFixed(1)} ${sy2.toFixed(1)} Q ${mx.toFixed(1)} ${my.toFixed(
            1,
        )} ${ex2.toFixed(1)} ${ey2.toFixed(1)}`;
        if (arrows && len > 14) {
            let tx = ex2 - mx;
            let ty = ey2 - my;
            const tl = Math.hypot(tx, ty) || 1;
            tx /= tl;
            ty /= tl;
            const ca = Math.cos(0.42);
            const sa = Math.sin(0.42);
            const aLen = 6;
            const b1x = ex2 + (-tx * ca + ty * sa) * aLen;
            const b1y = ey2 + (-tx * sa - ty * ca) * aLen;
            const b2x = ex2 + (-tx * ca - ty * sa) * aLen;
            const b2y = ey2 + (tx * sa - ty * ca) * aLen;
            d += ` M ${ex2.toFixed(1)} ${ey2.toFixed(1)} L ${b1x.toFixed(1)} ${b1y.toFixed(
                1,
            )} M ${ex2.toFixed(1)} ${ey2.toFixed(1)} L ${b2x.toFixed(1)} ${b2y.toFixed(1)}`;
        }
        return d;
    }

    function buildNeighbours(edges: DisplayEdge[]): Map<string, Set<string>> {
        const map = new Map<string, Set<string>>();
        for (const edge of edges) {
            if (!map.has(edge.prerequisiteId)) {
                map.set(edge.prerequisiteId, new Set());
            }
            if (!map.has(edge.targetId)) {
                map.set(edge.targetId, new Set());
            }
            map.get(edge.prerequisiteId)!.add(edge.targetId);
            map.get(edge.targetId)!.add(edge.prerequisiteId);
        }
        return map;
    }

    // Spotlight the active node's edges + neighbourhood; fade the rest.
    function isDim(edge: { from: string; to: string }, active: string | null): boolean {
        return active != null && edge.from !== active && edge.to !== active;
    }

    function isHot(edge: { from: string; to: string }, active: string | null): boolean {
        return active != null && (edge.from === active || edge.to === active);
    }

    function isFaded(
        id: string,
        active: string | null,
        nb: Map<string, Set<string>>,
    ): boolean {
        if (active == null || id === active) {
            return false;
        }
        return !(nb.get(active)?.has(id) ?? false);
    }

    function clampZoom(z: number): number {
        return Math.min(2.2, Math.max(0.4, z));
    }

    function zoomTo(target: number, ax?: number, ay?: number, animate = true): void {
        if (!graphEl) {
            void zoom.set(clampZoom(target), { duration: 0 });
            return;
        }
        const anchorX = ax ?? graphEl.clientWidth / 2;
        const anchorY = ay ?? graphEl.clientHeight / 2;
        zoomAnchor = {
            cx: graphEl.scrollLeft + anchorX,
            cy: graphEl.scrollTop + anchorY,
            ax: anchorX,
            ay: anchorY,
            z0: zoomVal,
        };
        void zoom.set(clampZoom(target), { duration: animate ? 220 : 0 });
    }

    // Keep the anchor point fixed as the zoom tween plays (nodes + edges glide
    // together; dot + label sizes stay constant, only spacing scales).
    function reflowScroll(z: number): void {
        if (!graphEl || !zoomAnchor) {
            return;
        }
        graphEl.scrollLeft = zoomAnchor.cx * (z / zoomAnchor.z0) - zoomAnchor.ax;
        graphEl.scrollTop = zoomAnchor.cy * (z / zoomAnchor.z0) - zoomAnchor.ay;
    }

    function fitTarget(): number {
        if (!graphEl) {
            return 1;
        }
        const availW = graphEl.clientWidth - PAD * 2 - LABELPAD;
        const availH = graphEl.clientHeight - PAD * 2 - ROWH;
        const zW = availW / Math.max((units.cols - 1) * COLW, 1);
        const zH = availH / Math.max((units.rows - 1) * ROWH, 1);
        return clampZoom(Math.min(zW, zH) || 1);
    }

    function fit(animate = true): void {
        zoomTo(fitTarget(), undefined, undefined, animate);
    }

    // ctrl/cmd + wheel zooms anchored at the cursor; plain wheel scrolls natively.
    function wheelZoom(node: HTMLElement) {
        const handler = (event: WheelEvent) => {
            if (!(event.ctrlKey || event.metaKey)) {
                return;
            }
            event.preventDefault();
            const rect = node.getBoundingClientRect();
            zoomTo(
                zoomVal * (event.deltaY < 0 ? 1.12 : 1 / 1.12),
                event.clientX - rect.left,
                event.clientY - rect.top,
                false,
            );
        };
        node.addEventListener("wheel", handler, { passive: false });
        return {
            destroy() {
                node.removeEventListener("wheel", handler);
            },
        };
    }

    async function selectTopic(node: { id: string; startable: boolean }): Promise<void> {
        if (!node.startable) {
            return;
        }
        try {
            await setConceptSelectedTopic({
                deckId: state.getTargetDeckId(),
                topic: node.id,
            });
        } catch {
            // Ignore — the status refresh below reflects whatever persisted.
        }
        await refreshStatus();
    }
</script>

<TitledContainer title={tr.deckConfigConceptScheduler()}>
    <DynamicallySlottable slotHost={Item} {api}>
        <Item>
            <SwitchRow
                bind:value={$limits.conceptSchedulerEnabled}
                defaultValue={false}
            >
                <SettingTitle>
                    {tr.deckConfigConceptSchedulerMode()}
                </SettingTitle>
            </SwitchRow>
            <div class="description">
                {tr.deckConfigConceptSchedulerModeTooltip()}
            </div>
        </Item>
        {#if $limits.conceptSchedulerEnabled}
            <Item>
                <div class="concept-summary">
                    <div class="status-text">
                        {tr.deckConfigConceptSchedulerStatusOn({
                            deck: state.currentDeck.name,
                        })}
                    </div>
                    {#if status?.counters}
                        <div class="counter-grid">
                            <div>
                                <strong>{evidencePercent(status)}</strong>
                                <span>{tr.deckConfigConceptSchedulerSeenCards()}</span>
                            </div>
                            <div>
                                <strong>{status.counters.dailyPositive}</strong>
                                <span>
                                    {tr.deckConfigConceptSchedulerPositiveToday()}
                                </span>
                            </div>
                            <div>
                                <strong>
                                    {status.counters.prerequisiteViolationsTotal}
                                </strong>
                                <span>
                                    {tr.deckConfigConceptSchedulerPrereqViolations()}
                                </span>
                            </div>
                        </div>
                        <div class="description">
                            Graph nodes: {status.graph?.nodes.length ?? 0}; node
                            answers:
                            {totalGraphAnswers()}; total evidence:
                            {status.counters.totalSeenCards}
                        </div>
                    {/if}

                    <!-- Concept map: the layered "builds on -> unlocks" graph -->
                    <div class="cg" aria-label="MCAT concept map">
                        <div class="cg-head">
                            <div class="cg-title">
                                <strong>Concept map</strong>
                                <span class="cg-sub">
                                    {readyCount} ready to start · {liveNodes.length}
                                    concepts
                                </span>
                            </div>
                            <span class="cg-controls">
                                <button
                                    type="button"
                                    title="Zoom out"
                                    on:click={() => zoomTo(zoomVal / 1.25)}
                                >
                                    &#8722;
                                </button>
                                <button
                                    type="button"
                                    title="Fit to view"
                                    on:click={() => fit(true)}
                                >
                                    Fit
                                </button>
                                <button
                                    type="button"
                                    title="Zoom in"
                                    on:click={() => zoomTo(zoomVal * 1.25)}
                                >
                                    +
                                </button>
                            </span>
                        </div>

                        {#if nodeViews.length}
                            <div
                                class="cg-graph"
                                class:dense={nodeViews.length > 16}
                                bind:this={graphEl}
                                use:wheelZoom
                            >
                                <div
                                    class="cg-canvas"
                                    class:entering={!didFit}
                                    style="width:{canvasW}px;height:{canvasH}px"
                                >
                                    <svg
                                        style="width:{canvasW}px;height:{canvasH}px"
                                        viewBox={`0 0 ${canvasW} ${canvasH}`}
                                    >
                                        {#each edgeViews as edge (edge.from + "->" + edge.to)}
                                            <path
                                                d={edge.d}
                                                class:dim={isDim(edge, activeId)}
                                                class:hot={isHot(edge, activeId)}
                                            />
                                        {/each}
                                    </svg>
                                    {#each nodeViews as node (node.id)}
                                        <button
                                            type="button"
                                            class="cg-node"
                                            class:locked={node.bucket === "locked"}
                                            class:available={node.startable}
                                            class:recommended={node.recommended}
                                            class:focus={node.current}
                                            class:hovered={hoverId === node.id}
                                            class:faded={isFaded(
                                                node.id,
                                                activeId,
                                                neighbourSet,
                                            )}
                                            class:nolabel={!showLabels}
                                            style="left:{nx(node.col, zoomVal)}px;top:{ny(
                                                node.row,
                                                zoomVal,
                                            )}px;--sec:{node.sec};--dot:{node.dotRem}rem;--m:{node.mastery}"
                                            title={`${node.subject} · ${node.name} · ${percent(
                                                node.mastery,
                                            )} mastered${node.current ? " · current" : ""}${
                                                node.recommended ? " · suggested next" : ""
                                            }${node.bucket === "inner" ? " · mastered" : ""}${
                                                node.startable ? " · click to start" : ""
                                            }`}
                                            aria-label={`${node.name}${
                                                node.startable
                                                    ? ", ready to start, select as next topic"
                                                    : ""
                                            }`}
                                            aria-pressed={node.current}
                                            tabindex={node.startable ? 0 : -1}
                                            on:mouseenter={() => (hoverId = node.id)}
                                            on:mouseleave={() => (hoverId = null)}
                                            on:focus={() => (hoverId = node.id)}
                                            on:blur={() => (hoverId = null)}
                                            on:click={() => selectTopic(node)}
                                        >
                                            <span class="cg-arc"></span>
                                            <span
                                                class="cg-dot"
                                                style="background:{node.sec};width:{node.dotRem}rem;height:{node.dotRem}rem"
                                            ></span>
                                            {#if node.recommended}
                                                <span class="cg-star">&#9733;</span>
                                            {/if}
                                            {#if node.bucket === "inner"}
                                                <span class="cg-check">&#10003;</span>
                                            {/if}
                                            <span class="cg-label">{node.label}</span>
                                        </button>
                                    {/each}
                                </div>
                            </div>
                            <div class="cg-legend" aria-hidden="true">
                                <span>
                                    <span class="cg-key" style="background:#4f74d6"></span>
                                    Bio/Biochem
                                </span>
                                <span>
                                    <span class="cg-key" style="background:#d65f5f"></span>
                                    Chem/Phys
                                </span>
                                <span>
                                    <span class="cg-key" style="background:#5aa469"></span>
                                    Psych/Soc
                                </span>
                                <span>
                                    <span class="cg-key" style="background:#c99a3a"></span>
                                    CARS
                                </span>
                                <span><span class="cg-key ready"></span>ready to start</span>
                                <span><span class="cg-glyph star">&#9733;</span>suggested</span>
                                <span><span class="cg-key current"></span>current</span>
                                <span><span class="cg-glyph check">&#10003;</span>mastered</span>
                                <span class="cg-hint">
                                    ring fills with mastery · size = importance · click a
                                    green node to pick it
                                </span>
                                <span class="cg-hint flow">
                                    builds on &#8594; this &#8594; unlocks
                                </span>
                            </div>
                        {:else}
                            <p class="cg-empty-msg">
                                No concept data yet — study a few cards in this deck to
                                build your map.
                            </p>
                        {/if}
                    </div>

                    {#if status}
                        <div class="backend-snapshot">
                            <div class="backend-snapshot-card">
                                <strong>Status response</strong>
                                <span>enabled: {String(status.enabled)}</span>
                                <span>active: {String(status.active)}</span>
                                <span>evidence kind: {evidenceKind(status)}</span>
                                <span>
                                    evidence:
                                    {status.evidence?.seenCards ?? 0}/{status.evidence
                                        ?.requiredSeenCards ?? 0}
                                    ({evidencePercent(status)})
                                </span>
                                <span>
                                    graph:
                                    {status.graph?.nodes.length ?? 0} nodes,
                                    {status.graph?.edges.length ?? 0} edges, cycle {String(
                                        status.graph?.hasCycle ?? false,
                                    )}
                                </span>
                            </div>
                            <div class="backend-snapshot-card">
                                <strong>Backend counters</strong>
                                <span>
                                    total seen: {status.counters?.totalSeenCards ?? 0}
                                </span>
                                <span>
                                    positive today: {status.counters?.dailyPositive ??
                                        0}
                                </span>
                                <span>
                                    negative today: {status.counters?.dailyNegative ??
                                        0}
                                </span>
                                <span>
                                    prereq violations today:
                                    {status.counters?.prerequisiteViolationsToday ?? 0}
                                </span>
                                <span>
                                    prereq violations total:
                                    {status.counters?.prerequisiteViolationsTotal ?? 0}
                                </span>
                                <span>
                                    memory (recall of studied cards):
                                    {status.hasMemory ? percent(status.overallMemory) : "—"}
                                </span>
                                <span>
                                    projected MCAT total (472-528):
                                    {status.hasProjection
                                        ? `${Math.round(status.projectedTotal)} (${Math.round(
                                              status.projectedTotalLower,
                                          )}–${Math.round(status.projectedTotalUpper)})`
                                        : "—"}
                                </span>
                            </div>
                            <div class="backend-snapshot-card">
                                <strong>Live queue session</strong>
                                {#if status.session}
                                    <span>
                                        slot progress:
                                        {status.session.reviewsTowardNextSlot}/{status
                                            .session.reviewsPerSlot}
                                        ({percent(status.session.budgetProgress)})
                                    </span>
                                    <span>
                                        slots available: {status.session.slotsAvailable}
                                    </span>
                                    <span>
                                        focused block:
                                        {status.session.blockRemaining}/{status.session
                                            .blockSize}
                                    </span>
                                    <span>
                                        active topic: {topicLabel(
                                            status.session.activeTopic,
                                        )}
                                    </span>
                                    <span>
                                        selected topic: {topicLabel(
                                            status.session.selectedTopic,
                                        )}
                                    </span>
                                {:else}
                                    <span>No active queue session yet</span>
                                {/if}
                            </div>
                            {#each status.sectionScores as score}
                                {@const scoresVisible =
                                    score.enoughEvidence && score.coverage >= 0.6}
                                {@const scoresHiddenReason =
                                    score.coverage < 0.6
                                        ? "needs 60% coverage"
                                        : "more evidence needed"}
                                <div class:insufficient={!score.enoughEvidence} class="backend-snapshot-card">
                                    <strong>{sectionName(score.section)}</strong>
                                    <span>
                                        Performance range: {scoresVisible
                                            ? scoreRange(score, "performance")
                                            : `— (${scoresHiddenReason})`}
                                    </span>
                                    <span>
                                        Readiness range: {scoresVisible
                                            ? scoreRange(score, "readiness")
                                            : `— (${scoresHiddenReason})`}
                                    </span>
                                    <span>Section mastery: {percent(score.sectionMastery)}</span>
                                    <span>Blueprint coverage: {percent(score.coverage)}</span>
                                    <span>
                                        Items answered: {score.answeredItems}/{score.requiredItems}
                                    </span>
                                    <span>
                                        Memory: {score.sectionHasMemory
                                            ? percent(score.sectionMemory)
                                            : "—"}
                                    </span>
                                </div>
                            {/each}
                        </div>
                        <div class="recommendation-list">
                            <strong>Backend recommendations</strong>
                            {#each status.recommendations as recommendation}
                                <div class="recommendation-row">
                                    <span>{recommendation.id}</span>
                                    <span>
                                        selectable {String(recommendation.selectable)}
                                    </span>
                                </div>
                            {:else}
                                <div class="recommendation-row empty">
                                    No backend recommendations yet
                                </div>
                            {/each}
                        </div>
                    {/if}
                    <div class="description">
                        {tr.deckConfigConceptSchedulerGraphLive()}
                    </div>
                </div>
            </Item>
        {:else}
            <Item>
                <div class="status-text">
                    {tr.deckConfigConceptSchedulerStatusOff()}
                </div>
            </Item>
        {/if}
    </DynamicallySlottable>
</TitledContainer>

<style lang="scss">
    :global(:root) {
        --concept-secondary-text: #456582;
        --concept-muted-text: #538263;
    }

    :global(.night-mode) {
        --concept-secondary-text: #c8e2f4;
        --concept-muted-text: #c8e8d0;
    }

    .description {
        color: var(--concept-secondary-text);
        font-size: var(--font-size-small);
        margin-top: 0.25rem;
    }

    .status-text {
        color: var(--fg);
        font-size: var(--font-size-small);
    }

    .concept-summary {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .backend-snapshot {
        display: grid;
        gap: 0.5rem;
        grid-template-columns: repeat(3, minmax(0, 1fr));
    }

    .backend-snapshot-card,
    .recommendation-list {
        background: #fbfaf7;
        border: 1px solid rgba(69 101 130 / 0.22);
        border-radius: var(--border-radius-medium);
        color: var(--fg);
        display: flex;
        flex-direction: column;
        font-size: var(--font-size-small);
        gap: 0.15rem;
        padding: 0.4rem;

        span {
            color: var(--concept-secondary-text);
        }
    }

    .backend-snapshot-card.insufficient {
        color: var(--concept-secondary-text);
    }

    .recommendation-row {
        align-items: baseline;
        border-top: 1px solid var(--border);
        display: flex;
        flex-wrap: wrap;
        gap: 0.15rem 0.55rem;
        padding-top: 0.25rem;

        span:first-child {
            color: var(--fg);
            font-weight: 600;
        }
    }

    .recommendation-row.empty {
        color: var(--concept-secondary-text);
        font-style: italic;
    }

    .counter-grid {
        display: grid;
        gap: 0.5rem;
        grid-template-columns: repeat(3, minmax(0, 1fr));

        div {
            background: #fbfaf7;
            border: 1px solid rgba(69 101 130 / 0.22);
            border-radius: var(--border-radius-medium);
            display: flex;
            flex-direction: column;
            font-size: var(--font-size-small);
            gap: 0.15rem;
            padding: 0.4rem;
        }

        span {
            color: var(--concept-secondary-text);
        }
    }

    // -- Concept map (shares the reviewer sidebar's visual language) -----------

    .cg {
        display: flex;
        flex-direction: column;
        gap: 0.4rem;
    }

    .cg-head {
        align-items: center;
        display: flex;
        gap: 0.5rem;
        justify-content: space-between;
    }

    .cg-title {
        display: flex;
        flex-direction: column;
    }

    .cg-title strong {
        font-weight: 600;
    }

    .cg-sub {
        color: var(--concept-secondary-text);
        font-size: var(--font-size-small);
    }

    .cg-controls {
        display: inline-flex;
        gap: 0.25rem;
    }

    .cg-controls button {
        background: transparent;
        border: 1px solid rgba(69 101 130 / 0.3);
        border-radius: 0.3rem;
        color: inherit;
        cursor: pointer;
        font-size: 12px;
        line-height: 1;
        min-width: 1.6rem;
        padding: 0.15rem 0.45rem;
    }

    .cg-controls button:hover {
        background: color-mix(in srgb, var(--canvas) 88%, #6f8fa8);
    }

    .cg-graph {
        background: color-mix(in srgb, #fbfaf7 94%, #6f8fa8);
        border: 1px solid rgba(69 101 130 / 0.24);
        border-radius: var(--border-radius-medium);
        height: 22rem;
        overflow: auto;
        position: relative;
    }

    .cg-graph.dense {
        height: 26rem;
    }

    .cg-canvas {
        position: relative;
        transform-origin: 0 0;
        transition: opacity 0.25s ease;
    }

    .cg-canvas.entering {
        opacity: 0;
    }

    .cg-graph svg {
        inset: 0;
        overflow: visible;
        position: absolute;
    }

    // Directed prerequisite -> target edges (curve + arrowhead share one path).
    .cg-graph svg path {
        fill: none;
        stroke: color-mix(in srgb, var(--concept-secondary-text) 52%, transparent);
        stroke-linecap: round;
        stroke-linejoin: round;
        stroke-width: 1.3;
        transition: opacity 0.15s ease, stroke 0.15s ease;
        vector-effect: non-scaling-stroke;
    }

    .cg-graph svg path.hot {
        stroke: color-mix(in srgb, var(--concept-secondary-text) 88%, transparent);
        stroke-width: 1.8;
    }

    .cg-graph svg path.dim {
        opacity: 0.12;
    }

    .cg-node {
        background: transparent;
        border: 0;
        cursor: default;
        padding: 0;
        position: absolute;
    }

    .cg-node.available {
        cursor: pointer;
    }

    // Mastery arc: a ring that fills (in the section colour) with node.mastery
    // over a faint track; the centre is masked out so only the annulus shows.
    .cg-arc {
        --track: color-mix(in srgb, var(--fg) 15%, transparent);
        background: conic-gradient(
            from -90deg,
            var(--sec, #4f74d6) 0 calc(var(--m, 0) * 360deg),
            var(--track) calc(var(--m, 0) * 360deg) 360deg
        );
        border-radius: 999px;
        height: calc(var(--dot, 0.6rem) + 0.6rem);
        left: 0;
        -webkit-mask: radial-gradient(circle, transparent 0 calc(50% - 2.2px), #000 calc(50% - 2.2px) 100%);
        mask: radial-gradient(circle, transparent 0 calc(50% - 2.2px), #000 calc(50% - 2.2px) 100%);
        pointer-events: none;
        position: absolute;
        top: 0;
        transform: translate(-50%, -50%);
        width: calc(var(--dot, 0.6rem) + 0.6rem);
        z-index: 0;
    }

    .cg-dot {
        border: 1px solid var(--border-strong);
        border-radius: 999px;
        box-shadow: 0 0 0 1px var(--border);
        left: 0;
        position: absolute;
        top: 0;
        transform: translate(-50%, -50%);
        transition: box-shadow 0.15s ease, transform 0.12s ease;
        z-index: 1;
    }

    // green ring = ready to start (prerequisites met, not yet mastered).
    .cg-node.available .cg-dot {
        box-shadow: 0 0 0 2px #2fbf6b, 0 0 7px rgba(47 191 107 / 0.5);
    }

    .cg-node.available:hover .cg-dot {
        box-shadow: 0 0 0 3px #2fbf6b, 0 0 12px rgba(47 191 107 / 0.9);
    }

    .cg-node.available:active .cg-dot {
        transform: translate(-50%, -50%) scale(0.86);
    }

    // Suggested next keeps the green ready ring and adds a pulsing gold glow + ★.
    .cg-node.recommended .cg-dot {
        animation: cg-suggest 1.6s ease-in-out infinite;
    }

    @keyframes cg-suggest {
        0%,
        100% {
            box-shadow: 0 0 0 2px #2fbf6b, 0 0 9px 2px rgba(242 194 0 / 0.5);
        }

        50% {
            box-shadow: 0 0 0 2.5px #2fbf6b, 0 0 16px 5px rgba(242 194 0 / 0.95);
        }
    }

    .cg-star {
        color: #f2c200;
        font-size: 0.8rem;
        left: 0;
        line-height: 1;
        pointer-events: none;
        position: absolute;
        text-shadow: 0 0 3px rgba(0 0 0 / 0.6), 0 0 2px rgba(0 0 0 / 0.8);
        top: 0;
        transform: translate(-115%, -115%);
        z-index: 3;
    }

    // violet pulsing ring = the current selected/active topic.
    .cg-node.focus .cg-dot {
        animation: cg-current 1.8s ease-in-out infinite;
        transform: translate(-50%, -50%) scale(1.28);
    }

    @keyframes cg-current {
        0%,
        100% {
            box-shadow:
                0 0 0 2.5px #7c5cff,
                0 0 0 4.5px color-mix(in srgb, var(--canvas) 65%, transparent),
                0 0 8px 1px rgba(124 92 255 / 0.6);
        }

        50% {
            box-shadow:
                0 0 0 3px #7c5cff,
                0 0 0 5px color-mix(in srgb, var(--canvas) 65%, transparent),
                0 0 20px 6px rgba(124 92 255 / 0.95);
        }
    }

    .cg-check {
        color: #2f9e5e;
        font-size: 0.62rem;
        font-weight: 700;
        line-height: 1;
        pointer-events: none;
        position: absolute;
        right: 0;
        text-shadow: 0 0 2px rgba(255 255 255 / 0.6);
        top: 0;
        transform: translate(135%, -135%);
        z-index: 3;
    }

    .cg-node.hovered {
        z-index: 5;
    }

    .cg-node.hovered .cg-dot {
        transform: translate(-50%, -50%) scale(1.22);
    }

    .cg-node.faded {
        opacity: 0.26;
        transition: opacity 0.15s ease;
    }

    .cg-node.faded .cg-label {
        display: none;
    }

    .cg-node.locked {
        opacity: 0.4;
    }

    .cg-label {
        background: color-mix(in srgb, #fbfaf7 90%, #fff);
        border: 1px solid color-mix(in srgb, var(--border) 70%, transparent);
        border-radius: 0.25rem;
        color: var(--fg);
        font-size: 10px;
        left: 0.55rem;
        line-height: 1.05;
        max-width: 7rem;
        overflow: hidden;
        padding: 0.04rem 0.18rem;
        pointer-events: none;
        position: absolute;
        text-overflow: ellipsis;
        top: 0;
        transform: translateY(-50%);
        white-space: nowrap;
    }

    .cg-node.nolabel .cg-label {
        display: none;
    }

    .cg-node.hovered.nolabel .cg-label {
        display: inline-block;
    }

    .cg-legend {
        align-items: center;
        color: var(--concept-secondary-text);
        display: flex;
        flex-wrap: wrap;
        font-size: 10px;
        gap: 0.5rem;
    }

    .cg-legend span {
        align-items: center;
        display: inline-flex;
        gap: 0.2rem;
    }

    .cg-key {
        border-radius: 999px;
        height: 0.5rem;
        width: 0.5rem;
    }

    .cg-key.ready {
        background: color-mix(in srgb, var(--canvas) 55%, #8a8a8a);
        box-shadow: 0 0 0 2px #2fbf6b;
    }

    .cg-key.current {
        background: color-mix(in srgb, var(--canvas) 55%, #8a8a8a);
        box-shadow: 0 0 0 2px #7c5cff, 0 0 5px 1px rgba(124 92 255 / 0.7);
    }

    .cg-glyph.check {
        color: #2f9e5e;
        font-weight: 700;
    }

    .cg-glyph.star {
        color: #f2c200;
        text-shadow: 0 0 2px rgba(0 0 0 / 0.5);
    }

    .cg-hint {
        color: var(--concept-secondary-text);
        flex-basis: 100%;
    }

    .cg-hint.flow {
        font-style: italic;
        opacity: 0.85;
    }

    .cg-empty-msg {
        color: var(--concept-secondary-text);
        font-size: var(--font-size-small);
        font-style: italic;
        margin: 0.5rem 0;
    }

    :global(.night-mode) {
        .backend-snapshot-card,
        .recommendation-list,
        .counter-grid div {
            background: var(--canvas-elevated);
            border-color: var(--border-subtle);
            color: var(--fg);
        }

        .backend-snapshot-card span,
        .recommendation-list span,
        .counter-grid span {
            color: var(--concept-muted-text);
        }

        .cg-graph {
            background: color-mix(in srgb, var(--canvas) 92%, #456582);
            border-color: rgba(143 184 154 / 0.28);
        }

        .cg-graph svg path {
            stroke: color-mix(in srgb, var(--concept-muted-text) 65%, transparent);
        }

        .cg-graph svg path.hot {
            stroke: color-mix(in srgb, var(--concept-muted-text) 95%, transparent);
        }

        .cg-arc {
            --track: color-mix(in srgb, var(--fg) 22%, transparent);
        }

        .cg-label {
            background: color-mix(in srgb, var(--canvas) 86%, #456582);
            border-color: rgba(143 184 154 / 0.3);
            color: var(--fg);
        }
    }

    @media (max-width: 700px) {
        .backend-snapshot,
        .counter-grid {
            grid-template-columns: 1fr;
        }
    }
</style>
