<script lang="ts">
    import { page } from '$app/stores';
    import { intent } from '$lib/stores/intent';
    import { goto } from '$app/navigation';
    import { bookmarks, toggleBookmark } from '$lib/stores/bookmarks';
    import { onMount } from 'svelte';

    // State
    let references = $state<any[]>([]);
    let previews = $state<Record<string, { citation: string, bibliography: string }>>({});
    let selectedCategory = $state('All');
    let loadingReferences = $state(true);

    const categories = ['All', 'Humanities', 'Social Sciences', 'Hard Sciences', 'Law', 'Edge Cases'];

    // Mock data for style metadata (still needed for header info)
    const stylesInfo: Record<string, any> = {
        'apa': {
            name: 'APA 7th Edition',
            description: 'The American Psychological Association style is most commonly used to cite sources within the social sciences.',
            version: 'v7.0.0',
            author: 'American Psychological Association',
            license: 'CC-BY-SA 3.0',
            id: 'http://www.zotero.org/styles/apa',
            tags: ['Social Sciences', 'Author-Date'],
            color: 'from-blue-700 to-blue-500'
        },
        'nature': {
            name: 'Nature',
            description: 'The citation style for the journal Nature. It is a numeric style used widely in the sciences.',
            version: 'v2.1',
            author: 'Nature Publishing Group',
            license: 'CC-BY-SA 3.0',
            id: 'http://www.zotero.org/styles/nature',
            tags: ['Science', 'Numeric'],
            color: 'from-emerald-700 to-emerald-500'
        },
        'chicago': {
            name: 'Chicago 17th Ed (Notes)',
            description: 'The Chicago Manual of Style (Notes and Bibliography) is used primarily in history and the humanities.',
            version: 'v17.0',
            author: 'University of Chicago Press',
            license: 'CC-BY-SA 3.0',
            id: 'http://www.zotero.org/styles/chicago-note-bibliography',
            tags: ['Humanities', 'Notes'],
            color: 'from-orange-700 to-orange-500'
        }
    };

    let styleId = $derived($page.params.id);
    let style = $derived(stylesInfo[styleId] || stylesInfo['apa']);

    let filteredReferences = $derived(
        selectedCategory === 'All' 
            ? references 
            : references.filter(ref => {
                const slug = selectedCategory.toLowerCase().replace(' ', '-');
                const kw = ref.keywords || [];
                // Special mapping for 'edge cases' -> 'edge-case'? yaml has 'edge-case' or 'edge-cases'? 
                // In file: "Edge Cases" section comment, but keywords? 
                // who_report: [medicine, global-health]. 
                // economist_editorial: [news, anonymous]. 
                // Actually the keywords in yaml for who_report are NOT 'edge-cases'.
                // Ideally backend categorization should be robust.
                // For now, let's just check if ANY keyword matches.
                // Or looking at file:
                // foucault: [humanities...]
                // berger_luckmann: [social-sciences...]
                // einstein: [hard-sciences...]
                // brown_v_board: [law]
                // who_report: [medicine...] -> Maybe map to Edge Cases manually or simply stick to the 3 main ones + law?
                
                if (slug === 'edge-cases') {
                     // Catch-all for things that don't match others?
                     return !kw.some((k: string) => ['humanities', 'social-sciences', 'hard-sciences', 'law'].includes(k));
                }
                
                return kw.includes(slug);
            })
    );

    function handleFork() {
        intent.update(i => ({ ...i, base_archetype: styleId }));
        goto('/create-wizard');
    }

    onMount(async () => {
        try {
            const res = await fetch('/api/references');
            const data = await res.json();
            // Convert map to array
            references = Object.entries(data).map(([id, ref]: [string, any]) => ({ ...ref, id }));
        } catch (e) {
            console.error("Failed to fetch references:", e);
        } finally {
            loadingReferences = false;
        }
    });

    function getStyleDefinition(id: string) {
        // Minimal CSLN definitions for previews
        if (id === 'apa') {
            return {
                version: "",
                info: { title: 'APA' },
                citation: { "use-preset": "apa", "wrap": "parentheses" },
                bibliography: { "use-preset": "apa" }
            };
        }
        if (id === 'nature') {
            // Nature is numeric. Using IEEE as proxy for now as CSLN core might not have Nature specific preset yet.
            // Or maybe it does? The user code for csln_core had Ieee, Vancouver.
            return {
                version: "",
                info: { title: 'Nature' },
                citation: { "use-preset": "ieee" },
                bibliography: { "use-preset": "ieee" }
            };
        }
        if (id === 'chicago') {
            return {
                version: "",
                info: { title: 'Chicago' },
                citation: { "use-preset": "chicago-author-date", "wrap": "parentheses" },
                bibliography: { "use-preset": "chicago-author-date" }
            };
        }
        // Default fallthrough
        return {
            version: "",
            info: { title: 'Unknown' },
            citation: { "use-preset": "apa", "wrap": "parentheses" },
            bibliography: { "use-preset": "apa" }
        };
    }

    async function updatePreviews(refs: any[], sId: string) {
        if (!refs.length) return;

        const styleDef = getStyleDefinition(sId);

        // Fetch previews one by one to avoid large payload block 
        // (or can do parallel)
        for (const ref of refs) {
            if (previews[ref.id]) continue; // Already cached? (Maybe clear on style change)

            // Citation
            fetch('/preview/citation', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ style: styleDef, references: [ref] })
            })
            .then(r => r.json())
            .then(data => {
                previews[ref.id] = { ...previews[ref.id], citation: data.result };
            });

            // Bibliography
            fetch('/preview/bibliography', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ style: styleDef, references: [ref] })
            })
            .then(r => r.json())
            .then(data => {
                previews[ref.id] = { ...previews[ref.id], bibliography: data.result };
            });
        }
    }

    $effect(() => {
        // Clear previews when style changes
        // previews = {}; // Actually don't clear all, just maybe re-fetch?
        // If style ID changes, we need to re-fetch EVERYTHING.
        // But if filtering changes, we just need to fetch visible ones.
        
        // Simple strategy: Clear previews if styleId changes distinct from previous (tracking logic needed or just trust reactivity)
        // Since we don't strictly track prev styleId easily in effect without extra var, 
        // we'll just re-request. The browser cache might help, or we brute force it.
        // Let's reset previews when styleId changes. 
        // But $effect runs on any dep change. 
        // If we clear previews here, we might loop if previews is a dep?
        // references and styleId are deps explicitly or implicitly.
        
        // We pass visible filtered references
        if(filteredReferences.length > 0) {
             // We need to know if the STYLE changed to invalidate current previews.
             // For now, let's just make the request. 
             // We'll modify updatePreviews to force update?
             // Actually, the key is the styleId.
             // Let's clear previews inside updatePreviews if necessary? No.
             
             // Just call:
             updatePreviews(filteredReferences, styleId);
        }
    });
    
    // Watch styleId separately to clear cache?
    let lastStyleId = $state(styleId);
    $effect(() => {
        if (styleId !== lastStyleId) {
            previews = {};
            lastStyleId = styleId;
        }
    });

    function getTitle(ref: any) {
        if (typeof ref.title === 'string') return ref.title;
        if (ref.title?.main) return ref.title.main;
        return "Untitled";
    }

    function getColorForType(type: string) {
        switch (type) {
            case 'article-journal': return 'bg-emerald-500';
            case 'book': return 'bg-orange-500';
            case 'chapter': return 'bg-blue-500';
            case 'report': return 'bg-purple-500';
            case 'webpage': return 'bg-pink-500';
            default: return 'bg-slate-500';
        }
    }
</script>

<main class="mx-auto max-w-[1400px] px-6 py-12">
        <!-- Breadcrumbs -->
        <nav class="flex items-center gap-2 mb-8 text-sm text-slate-400 font-medium">
            <a href="/" class="hover:text-primary transition-colors">Library</a>
            <span class="material-symbols-outlined text-sm">chevron_right</span>
            <span class="text-slate-900">{style.name}</span>
        </nav>

        <!-- Header -->
        <div class="flex flex-col lg:flex-row lg:items-end justify-between gap-8 pb-10 border-b border-slate-200">
            <div class="max-w-3xl">
                <div class="flex items-center gap-3 mb-4">
                    <span class="px-2.5 py-0.5 rounded-full bg-blue-50 text-blue-600 text-[10px] font-black uppercase tracking-wider border border-blue-100">Verified</span>
                    {#each style.tags as tag}
                        <span class="px-2.5 py-0.5 rounded-full bg-slate-100 text-slate-500 text-[10px] font-black uppercase tracking-wider">{tag}</span>
                    {/each}
                </div>
                <h1 class="text-5xl font-black text-slate-950 tracking-tight leading-none mb-4">
                    {style.name}
                </h1>
                <p class="text-xl text-slate-500 leading-relaxed font-medium">
                    {style.description}
                </p>
            </div>
            <div class="flex gap-3">
                <button 
                    onclick={() => toggleBookmark(styleId)}
                    class="flex items-center gap-2 h-12 px-6 rounded-xl border border-slate-200 {$bookmarks.includes(styleId) ? 'text-primary bg-blue-50 border-primary/20' : 'bg-white text-slate-950'} font-bold hover:bg-slate-50 transition-all"
                >
                    <span class="material-symbols-outlined {$bookmarks.includes(styleId) ? 'fill-1' : ''}">
                        {$bookmarks.includes(styleId) ? 'bookmark' : 'star'}
                    </span>
                    {$bookmarks.includes(styleId) ? 'Saved' : 'Star'}
                </button>
                <button 
                    onclick={handleFork}
                    class="flex items-center gap-2 h-12 px-6 rounded-xl bg-slate-950 text-white font-bold hover:bg-slate-800 transition-all shadow-xl shadow-slate-200"
                >
                    <span class="material-symbols-outlined">fork_right</span>
                    Fork & Edit
                </button>
            </div>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-12 gap-12 mt-12">
            <!-- Sidebar -->
            <aside class="lg:col-span-4 flex flex-col gap-8">
                <div class="rounded-3xl border border-slate-200 bg-white p-8 shadow-sm">
                    <div class="flex flex-col gap-6">
                        <div>
                            <h3 class="text-[10px] font-black uppercase tracking-widest text-slate-400 mb-4">Current Version</h3>
                            <div class="flex items-baseline gap-2">
                                <span class="text-4xl font-black text-slate-950">{style.version}</span>
                                <span class="px-2 py-0.5 rounded-md bg-emerald-50 text-emerald-600 text-[10px] font-black uppercase tracking-wider border border-emerald-100">Stable</span>
                            </div>
                            <p class="text-sm text-slate-400 font-medium mt-1 text-slate-500">Updated Feb 03, 2024</p>
                        </div>
                        
                        <div class="space-y-3">
                            <button class="w-full flex items-center justify-center gap-2 h-11 bg-primary text-white font-bold rounded-xl hover:bg-blue-600 transition-all shadow-lg shadow-blue-100">
                                <span class="material-symbols-outlined text-[20px]">download</span>
                                Download CSL
                            </button>
                            <button class="w-full flex items-center justify-center gap-2 h-11 bg-white border border-slate-200 text-slate-950 font-bold rounded-xl hover:bg-slate-50 transition-all">
                                <span class="material-symbols-outlined text-[20px]">code</span>
                                Copy JSON
                            </button>
                        </div>

                        <div class="pt-6 border-t border-slate-100 space-y-4">
                            <div class="flex items-start gap-3">
                                <span class="material-symbols-outlined text-slate-300">verified_user</span>
                                <div>
                                    <p class="text-xs font-black uppercase tracking-widest text-slate-400">Maintained by</p>
                                    <p class="font-bold text-slate-900">{style.author}</p>
                                </div>
                            </div>
                            <div class="flex items-start gap-3">
                                <span class="material-symbols-outlined text-slate-300">policy</span>
                                <div>
                                    <p class="text-xs font-black uppercase tracking-widest text-slate-400">License</p>
                                    <p class="font-bold text-slate-900">{style.license}</p>
                                </div>
                            </div>
                            <div class="flex items-start gap-3">
                                <span class="material-symbols-outlined text-slate-300">link</span>
                                <div class="overflow-hidden">
                                    <p class="text-xs font-black uppercase tracking-widest text-slate-400">Canonical ID</p>
                                    <p class="font-mono text-[10px] text-slate-500 truncate mt-1">{style.id}</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="rounded-3xl border border-slate-200 bg-white p-8 shadow-sm">
                    <h3 class="text-[10px] font-black uppercase tracking-widest text-slate-400 mb-4">Compatibility</h3>
                    <div class="flex flex-wrap gap-2">
                        {#each ['Zotero', 'Mendeley', 'Papers', 'RefWorks'] as tool}
                            <span class="px-3 py-1.5 rounded-lg bg-slate-50 border border-slate-100 text-slate-900 text-xs font-bold">{tool}</span>
                        {/each}
                    </div>
                </div>
            </aside>

    <section class="lg:col-span-8 space-y-8">
        <div class="flex flex-col gap-6 mb-4">
            <div class="flex items-center justify-between">
                <h2 class="text-2xl font-black text-slate-950 tracking-tight">Citation stress tests</h2>
                <div class="text-sm text-slate-500 font-bold">
                    {filteredReferences.length} tests
                </div>
            </div>

            <!-- Category Filters -->
            <div class="flex flex-wrap gap-2">
                {#each categories as category}
                    <button 
                        class="px-4 py-2 rounded-lg text-xs font-bold transition-all {selectedCategory === category ? 'bg-slate-900 text-white shadow-lg shadow-slate-200' : 'bg-white border border-slate-200 text-slate-600 hover:bg-slate-50 hover:border-slate-300'}"
                        onclick={() => selectedCategory = category}
                    >
                        {category}
                    </button>
                {/each}
            </div>
        </div>

        {#if loadingReferences}
            <div class="py-20 text-center">
                <div class="inline-block w-8 h-8 border-4 border-slate-200 border-t-primary rounded-full animate-spin mb-4"></div>
                <p class="text-slate-500 font-medium">Loading test cases...</p>
            </div>
        {:else if filteredReferences.length === 0}
             <div class="py-20 text-center bg-slate-50 rounded-3xl border border-slate-100">
                <span class="material-symbols-outlined text-4xl text-slate-300 mb-3">category</span>
                <p class="text-slate-500 font-medium">No examples found for {selectedCategory}</p>
            </div>
        {:else}
            {#each filteredReferences as ref (ref.id)}
                <article class="rounded-3xl border border-slate-200 bg-white overflow-hidden shadow-sm hover:shadow-md transition-shadow">
                    <div class="px-8 py-5 bg-slate-50/50 border-b border-slate-100 flex items-center justify-between">
                        <div class="flex items-center gap-3">
                            <div class="w-2 h-2 rounded-full {getColorForType(ref.type)}"></div>
                            <h3 class="font-bold text-slate-900 truncate max-w-[300px]" title={getTitle(ref)}>{getTitle(ref)}</h3>
                        </div>
                        <span class="text-[10px] font-mono text-slate-400 bg-white px-2 py-1 rounded border border-slate-100">{ref.type}</span>
                    </div>
                    <div class="p-8 space-y-8">
                        <div>
                            <p class="text-[10px] font-black uppercase tracking-widest text-slate-400 mb-4">In-Text Citation</p>
                            <div class="p-6 rounded-2xl bg-slate-50 font-serif text-lg leading-relaxed text-slate-800 border border-slate-100 min-h-[80px] flex items-center">
                                {#if previews[ref.id]?.citation}
                                    <span class="bg-blue-600/10 text-primary px-1 rounded mx-0.5 border-b border-primary/20">
                                        {@html previews[ref.id].citation}
                                    </span>
                                {:else}
                                    <span class="text-slate-400 text-sm animate-pulse">Generating preview...</span>
                                {/if}
                            </div>
                        </div>
                        <div>
                            <div class="flex items-center justify-between mb-4">
                                <p class="text-[10px] font-black uppercase tracking-widest text-slate-400">Bibliography Entry</p>
                            </div>
                            <div class="p-6 rounded-2xl bg-slate-50 font-serif text-lg leading-relaxed text-slate-800 border border-slate-100 min-h-[80px] flex items-center">
                                {#if previews[ref.id]?.bibliography}
                                    <div class="w-full">
                                        {@html previews[ref.id].bibliography}
                                    </div>
                                {:else}
                                    <span class="text-slate-400 text-sm animate-pulse">Generating preview...</span>
                                {/if}
                            </div>
                        </div>
                    </div>
                </article>
            {/each}
        {/if}
    </section>
        </div>
</main>
