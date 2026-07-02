<!--
Copyright: Ankitects Pty Ltd and contributors
License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html
-->
<script lang="ts">
    import { getConceptSchedulerStatus } from "@generated/backend";
    import * as tr from "@generated/ftl";
    import { onMount } from "svelte";

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
    type GraphPoint = { x: number; y: number };
    type MajorAreaId = "Bio" | "Biochem" | "ChemPhys" | "PsychSoc" | "CARS";

    const majorAreas: { id: MajorAreaId; label: string }[] = [
        { id: "Bio", label: "Bio" },
        { id: "Biochem", label: "Biochem" },
        { id: "ChemPhys", label: "Chem/Phys" },
        { id: "PsychSoc", label: "Psych/Soc" },
        { id: "CARS", label: "CARS" },
    ];

    const graphLayout: Record<string, GraphPoint> = {
        "Bio::DNA": { x: 8, y: 18 },
        "Bio::Genetics": { x: 30, y: 18 },
        "Bio::Eukaryotic_Cell": { x: 8, y: 72 },
        "Biochem::Amino_Acids": { x: 8, y: 45 },
        "Biochem::Peptides_and_Proteins": { x: 30, y: 45 },
        "Biochem::Protein_Structure_and_Function": { x: 52, y: 45 },
        "Biochem::Enzymes": { x: 72, y: 45 },
        "Biochem::Bioenergetics": { x: 52, y: 72 },
        "Biochem::Glycolysis": { x: 72, y: 72 },
        "Biochem::Citric_Acid_Cycle": { x: 90, y: 72 },
    };

    const fallbackEdges: DisplayEdge[] = [
        { prerequisiteId: "Bio::DNA", targetId: "Bio::Genetics" },
        {
            prerequisiteId: "Biochem::Amino_Acids",
            targetId: "Biochem::Peptides_and_Proteins",
        },
        {
            prerequisiteId: "Biochem::Peptides_and_Proteins",
            targetId: "Biochem::Protein_Structure_and_Function",
        },
        {
            prerequisiteId: "Biochem::Protein_Structure_and_Function",
            targetId: "Biochem::Enzymes",
        },
        {
            prerequisiteId: "Biochem::Enzymes",
            targetId: "Biochem::Bioenergetics",
        },
        {
            prerequisiteId: "Biochem::Bioenergetics",
            targetId: "Biochem::Glycolysis",
        },
        {
            prerequisiteId: "Biochem::Bioenergetics",
            targetId: "Biochem::Citric_Acid_Cycle",
        },
        {
            prerequisiteId: "Biochem::Glycolysis",
            targetId: "Biochem::Citric_Acid_Cycle",
        },
        {
            prerequisiteId: "Bio::Eukaryotic_Cell",
            targetId: "Biochem::Bioenergetics",
        },
    ];

    let status: ConceptStatus | null = null;
    let liveNodes: ConceptNode[] = [];
    let expandedAreas: Record<MajorAreaId, boolean> = {
        Bio: true,
        Biochem: true,
        ChemPhys: false,
        PsychSoc: false,
        CARS: false,
    };

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

    type FringeBucket = "inner" | "outer" | "locked";

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

    function majorAreaFor(topic: string): MajorAreaId {
        if (topic.startsWith("Bio::")) {
            return "Bio";
        }
        if (topic.startsWith("Biochem::")) {
            return "Biochem";
        }
        if (
            topic.startsWith("GenChem::") ||
            topic.startsWith("Physics::") ||
            topic.startsWith("Orgo::")
        ) {
            return "ChemPhys";
        }
        if (topic.startsWith("PsychSoc::")) {
            return "PsychSoc";
        }
        return "CARS";
    }

    function setAreaExpanded(area: MajorAreaId, expanded: boolean): void {
        expandedAreas = { ...expandedAreas, [area]: expanded };
    }

    function graphNodeIds(): string[] {
        const liveIds = status?.graph?.nodes.map((node) => node.id) ?? [];

        return liveIds.length ? liveIds : Object.keys(graphLayout);
    }

    function graphNodeIdsForArea(area: MajorAreaId): string[] {
        return graphNodeIds().filter((id) => majorAreaFor(id) === area);
    }

    function graphNodesForArea(area: MajorAreaId): ConceptNode[] {
        return liveNodes.filter((node) => majorAreaFor(node.id) === area);
    }

    function nodesForAreaAndFringe(
        area: MajorAreaId,
        fringe: FringeBucket,
    ): ConceptNode[] {
        return graphNodesForArea(area).filter((node) => fringeFor(node) === fringe);
    }

    function graphEdges(): DisplayEdge[] {
        const liveEdges =
            status?.graph?.edges
                .map((edge) => ({
                    prerequisiteId: edge.prerequisiteId,
                    targetId: edge.targetId,
                })) ?? [];

        return liveEdges.length ? liveEdges : fallbackEdges;
    }

    function graphEdgesForArea(area: MajorAreaId): DisplayEdge[] {
        const visibleIds = new Set(graphNodeIdsForArea(area));

        return graphEdges().filter(
            (edge) =>
                visibleIds.has(edge.prerequisiteId) && visibleIds.has(edge.targetId),
        );
    }

    function areaAnswerCount(area: MajorAreaId): number {
        return graphNodesForArea(area).reduce(
            (total, node) => total + answeredCount(node),
            0,
        );
    }

    function areaFringeCount(area: MajorAreaId, fringe: FringeBucket): number {
        return nodesForAreaAndFringe(area, fringe).length;
    }

    function pointFor(id: string, index = 0): GraphPoint {
        return (
            graphLayout[id] ?? {
                x: 12 + (index % 4) * 25,
                y: 20 + Math.floor(index / 4) * 25,
            }
        );
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

    function positiveCount(node: ConceptNode): number {
        return numberField(node, "positive", "positiveCount", "positive_count");
    }

    function negativeCount(node: ConceptNode): number {
        return numberField(node, "negative", "negativeCount", "negative_count");
    }

    function masteryScore(node: ConceptNode): number {
        return numberField(node, "mastery");
    }

    function compactMasteryText(node: ConceptNode): string {
        return `P ${percent(masteryScore(node))}`;
    }

    function fullMasteryText(node: ConceptNode): string {
        return `P mastery ${percent(masteryScore(node))}`;
    }

    function compactEvidenceText(node: ConceptNode): string {
        return `${answeredCount(node)} answered`;
    }

    function fullEvidenceText(node: ConceptNode): string {
        return `${positiveCount(node)} positive / ${negativeCount(node)} negative`;
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

    function scoreRange(score: ConceptSectionScore, prefix: "performance" | "readiness"): string {
        const record = score as unknown as Record<string, unknown>;
        const lower = Number(record[`${prefix}Lower`] ?? 0);
        const upper = Number(record[`${prefix}Upper`] ?? 0);

        return `${Math.round(lower)}-${Math.round(upper)}`;
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
                                <div class:insufficient={!score.enoughEvidence} class="backend-snapshot-card">
                                    <strong>{sectionName(score.section)}</strong>
                                    {#if score.enoughEvidence && score.coverage >= 0.6}
                                        <span>
                                            Performance range: {scoreRange(score, "performance")}
                                        </span>
                                        <span>
                                            Readiness range: {scoreRange(score, "readiness")}
                                        </span>
                                    {:else}
                                        {#if score.coverage < 0.6}
                                            <span>needs 60% coverage before scores show</span>
                                        {:else}
                                            <span>score hidden until more evidence is available</span>
                                        {/if}
                                    {/if}
                                    <span>Blueprint coverage: {percent(score.coverage)}</span>
                                    {#if score.sectionHasMemory}
                                        <span>Memory: {percent(score.sectionMemory)}</span>
                                    {/if}
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
                    <div class="graph-sections" aria-label="MCAT concept graph sections">
                        {#each majorAreas as area}
                            <details
                                class="graph-section"
                                open={expandedAreas[area.id]}
                                on:toggle={(event) =>
                                    setAreaExpanded(area.id, event.currentTarget.open)}
                            >
                                <summary>
                                    <strong>{area.label}</strong>
                                    <span>{graphNodeIdsForArea(area.id).length} topics</span>
                                    <span>{areaAnswerCount(area.id)} answered</span>
                                    <span>
                                        {areaFringeCount(area.id, "inner")} ready /
                                        {areaFringeCount(area.id, "outer")} next /
                                        {areaFringeCount(area.id, "locked")} locked
                                    </span>
                                </summary>
                                {#if expandedAreas[area.id]}
                                    <div class="concept-graph-viewport" aria-label={`${area.label} concept graph`}>
                                        <div class="concept-graph">
                                            <svg viewBox="0 0 100 100" preserveAspectRatio="none">
                                                {#each graphEdgesForArea(area.id) as edge}
                                                    {@const start = pointFor(edge.prerequisiteId)}
                                                    {@const end = pointFor(edge.targetId)}
                                                    <line
                                                        x1={start.x}
                                                        y1={start.y}
                                                        x2={end.x}
                                                        y2={end.y}
                                                    />
                                                {/each}
                                            </svg>
                                            {#if liveNodes.length}
                                                {#each graphNodesForArea(area.id) as node, index}
                                                    {@const point = pointFor(node.id, index)}
                                                    <div
                                                        class="graph-node {fringeFor(node)}"
                                                        style:left={`${point.x}%`}
                                                        style:top={`${point.y}%`}
                                                        title={node.id}
                                                    >
                                                        <strong>{node.id}</strong>
                                                        <span>{compactMasteryText(node)}</span>
                                                        <span>{compactEvidenceText(node)}</span>
                                                        <span>{positiveCount(node)} positive</span>
                                                    </div>
                                                {/each}
                                            {:else}
                                                {#each graphNodeIdsForArea(area.id) as id, index}
                                                    {@const point = pointFor(id, index)}
                                                    <div
                                                        class="graph-node locked"
                                                        style:left={`${point.x}%`}
                                                        style:top={`${point.y}%`}
                                                        title={id}
                                                    >
                                                        <strong>{id}</strong>
                                                        <span>P hidden</span>
                                                        <span>0 answered</span>
                                                    </div>
                                                {/each}
                                            {/if}
                                        </div>
                                    </div>
                                    <div class="concept-lattice" aria-hidden="true">
                                        <div class="concept-column">
                                            <div class="column-title">
                                                {tr.deckConfigConceptSchedulerInnerFringe()}
                                            </div>
                                            {#each nodesForAreaAndFringe(area.id, "inner") as node}
                                                <div class="concept-node inner" title={node.id}>
                                                    <strong>{node.id}</strong>
                                                    <span>{fullMasteryText(node)}</span>
                                                    <span>{compactEvidenceText(node)}</span>
                                                    <span>{fullEvidenceText(node)}</span>
                                                </div>
                                            {:else}
                                                <div class="concept-node empty">No topics yet</div>
                                            {/each}
                                        </div>
                                        <div class="concept-column">
                                            <div class="column-title">
                                                {tr.deckConfigConceptSchedulerOuterFringe()}
                                            </div>
                                            {#each nodesForAreaAndFringe(area.id, "outer") as node}
                                                <div class="concept-node outer" title={node.id}>
                                                    <strong>{node.id}</strong>
                                                    <span>{fullMasteryText(node)}</span>
                                                    <span>{compactEvidenceText(node)}</span>
                                                    <span>{fullEvidenceText(node)}</span>
                                                </div>
                                            {:else}
                                                <div class="concept-node empty">No topics yet</div>
                                            {/each}
                                        </div>
                                        <div class="concept-column">
                                            <div class="column-title">
                                                {tr.deckConfigConceptSchedulerLockedTopics()}
                                            </div>
                                            {#each nodesForAreaAndFringe(area.id, "locked") as node}
                                                <div class="concept-node locked" title={node.id}>
                                                    <strong>{node.id}</strong>
                                                    <span>{fullMasteryText(node)}</span>
                                                    <span>{compactEvidenceText(node)}</span>
                                                    <span>{fullEvidenceText(node)}</span>
                                                </div>
                                            {:else}
                                                <div class="concept-node empty">No topics yet</div>
                                            {/each}
                                        </div>
                                    </div>
                                {/if}
                            </details>
                        {/each}
                    </div>
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

    .column-title {
        font-weight: 600;
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

    .graph-sections {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .graph-section {
        background: color-mix(in srgb, #fbfaf7 94%, #8fb89a);
        border: 1px solid rgba(69 101 130 / 0.22);
        border-radius: var(--border-radius-medium);
        padding: 0.4rem;

        summary {
            align-items: center;
            color: var(--fg);
            cursor: pointer;
            display: flex;
            flex-wrap: wrap;
            font-size: var(--font-size-small);
            gap: 0.25rem 0.75rem;
        }

        summary span {
            color: var(--concept-secondary-text);
        }
    }

    .concept-graph-viewport {
        background: color-mix(in srgb, #fbfaf7 94%, #8fb89a);
        border: 1px solid rgba(69 101 130 / 0.24);
        border-radius: var(--border-radius-medium);
        max-height: 16rem;
        overflow: auto;
    }

    .concept-graph {
        height: 20rem;
        min-width: 38rem;
        position: relative;

        svg {
            height: 100%;
            inset: 0;
            position: absolute;
            width: 100%;
        }

        line {
            stroke: rgba(69 101 130 / 0.38);
            stroke-width: 0.8;
            vector-effect: non-scaling-stroke;
        }
    }

    .graph-node {
        background: #fbfaf7;
        border: 1px solid rgba(69 101 130 / 0.24);
        border-radius: 0.35rem;
        color: var(--fg);
        display: flex;
        flex-direction: column;
        font-size: 10px;
        gap: 0.05rem;
        line-height: 1.15;
        max-width: 5.5rem;
        padding: 0.18rem 0.22rem;
        position: absolute;
        text-align: center;
        transform: translate(-50%, -50%);

        strong {
            overflow-wrap: anywhere;
        }
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

    .concept-lattice {
        display: grid;
        gap: 0.75rem;
        grid-template-columns: repeat(3, minmax(0, 1fr));
        margin-top: 0.25rem;
    }

    .concept-column {
        display: flex;
        flex-direction: column;
        gap: 0.4rem;
        min-width: 0;
    }

    .concept-node {
        background: #fbfaf7;
        border: 1px solid rgba(69 101 130 / 0.22);
        border-radius: var(--border-radius-medium);
        color: var(--fg);
        display: flex;
        flex-direction: column;
        font-size: var(--font-size-small);
        gap: 0.15rem;
        min-width: 0;
        padding: 0.4rem;

        strong,
        span {
            overflow-wrap: anywhere;
        }
    }

    .concept-node.empty {
        border-left: 0;
        color: var(--concept-secondary-text);
        font-style: italic;
    }

    .inner {
        border-left: 0.25rem solid #456582;
    }

    .outer {
        border-left: 0.25rem solid #538263;
    }

    .locked {
        border-left: 0.16rem solid #d69a2d;
        color: var(--concept-secondary-text);
    }

    :global(.night-mode) {
        .backend-snapshot-card,
        .recommendation-list,
        .counter-grid div,
        .graph-section,
        .graph-node,
        .concept-node {
            background: var(--canvas-elevated);
            border-color: var(--border-subtle);
            color: var(--fg);
        }

        .concept-graph-viewport {
            background: color-mix(in srgb, var(--canvas) 92%, #456582);
            border-color: rgba(143 184 154 / 0.32);
        }

        .backend-snapshot-card span,
        .recommendation-list span,
        .graph-section summary span,
        .counter-grid span,
        .concept-node span {
            color: var(--concept-muted-text);
        }

        .concept-graph line {
            stroke: rgba(143 184 154 / 0.50);
        }
    }

    @media (max-width: 700px) {
        .backend-snapshot,
        .concept-lattice,
        .counter-grid {
            grid-template-columns: 1fr;
        }

    }
</style>
