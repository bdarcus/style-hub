<script lang="ts">
    import { intent } from '$lib/stores/intent';
    import { bookmarks, toggleBookmark } from '$lib/stores/bookmarks';
    import { onMount } from 'svelte';

    let searchQuery = $state('');
    let previews = $state<Record<string, string>>({});

    const allStyles = [
        { 
            id: 'apa', 
            name: 'APA 7th Edition', 
            type: 'Social Sciences • Standard', 
            color: 'from-blue-700 to-blue-500', 
            verified: true,
            preview: '<span class="group-hover:text-primary transition-colors font-medium">Doe, J.</span> (2024). <i>The Future of Citation.</i> Academic Press.' 
        },
        { 
            id: 'nature', 
            name: 'Nature', 
            type: 'Science • v2.0', 
            color: 'from-emerald-700 to-emerald-500', 
            verified: false,
            preview: '1. <span class="group-hover:text-primary transition-colors font-medium">Doe, J.</span> The Future of Citation. <i>Nature</i> (2024).' 
        },
        { 
            id: 'chicago', 
            name: 'Chicago (Notes)', 
            type: 'Humanities • 17th Ed', 
            color: 'from-orange-700 to-orange-500', 
            verified: false,
            preview: '<span class="group-hover:text-primary transition-colors font-medium">Doe, J.</span> 2024. <i>The Future of Citation.</i> New York.' 
        }
    ];

    let filteredStyles = $derived(
        searchQuery 
            ? allStyles.filter(s => s.name.toLowerCase().includes(searchQuery.toLowerCase()) || s.type.toLowerCase().includes(searchQuery.toLowerCase()))
            : allStyles
    );

    function getStyleDefinition(id: string) {
        if (id === 'apa') return { version: "", info: { title: 'APA' }, citation: { "use-preset": "apa", "wrap": "parentheses" }, bibliography: { "use-preset": "apa" } };
        if (id === 'nature') return { version: "", info: { title: 'Nature' }, citation: { "use-preset": "ieee" }, bibliography: { "use-preset": "ieee" } };
        if (id === 'chicago') return { version: "", info: { title: 'Chicago' }, citation: { "use-preset": "chicago-author-date", "wrap": "parentheses" }, bibliography: { "use-preset": "chicago-author-date" } };
        return { version: "", info: { title: 'Unknown' }, citation: { "use-preset": "apa", "wrap": "parentheses" }, bibliography: { "use-preset": "apa" } };
    }

    onMount(async () => {
        try {
            const res = await fetch('/api/references');
            const data = await res.json();
            const refList = Object.entries(data).map(([id, ref]: [string, any]) => ({ ...ref, id }));
            
            if (refList.length > 0) {
                 // Pick a book for better citation variety
                 const ref = refList.find(r => r.type === 'book') || refList[0];
                 
                 for (const style of allStyles) {
                     const styleDef = getStyleDefinition(style.id);
                     fetch('/preview/citation', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({ style: styleDef, references: [ref] })
                     })
                     .then(r => r.json())
                     .then(data => {
                         if (data.result) previews[style.id] = data.result;
                     });
                 }
            }
        } catch (e) {
            console.error("Failed to fetch references or previews", e);
        }
    });
</script>

<!-- Hero Section -->
<section class="relative bg-background-light py-12 lg:py-20 px-4 overflow-hidden">
    <!-- Background abstract graphic -->
    <div class="absolute inset-0 z-0 overflow-hidden pointer-events-none opacity-20">
        <div class="absolute -top-[20%] -right-[10%] w-[600px] h-[600px] rounded-full bg-primary blur-[120px]"></div>
        <div class="absolute top-[40%] -left-[10%] w-[400px] h-[400px] rounded-full bg-blue-400 blur-[100px]"></div>
    </div>
    
    <div class="relative z-10 flex flex-col items-center justify-center max-w-[960px] mx-auto text-center gap-6">
        <div class="flex flex-col gap-4">
            <h1 class="text-4xl lg:text-6xl font-black leading-tight tracking-[-0.033em] bg-clip-text text-transparent bg-gradient-to-br from-slate-900 to-slate-600">
                The Next Generation of <br/> CSL Citation Styles
            </h1>
            <h2 class="text-lg lg:text-xl font-normal text-slate-600 max-w-2xl mx-auto leading-relaxed">
                Find, edit, and generate citation styles with real-time academic previews. 
                Ensure your research meets the highest standards.
            </h2>
        </div>

        <!-- Search Box -->
        <div class="w-full max-w-[580px] mt-4">
            <div class="flex w-full items-center rounded-xl bg-white border border-slate-200 shadow-xl shadow-slate-200/50 p-2 focus-within:ring-2 focus-within:ring-primary/50 transition-all">
                <div class="flex items-center justify-center pl-3 text-slate-400">
                    <span class="material-symbols-outlined">search</span>
                </div>
                <input 
                    bind:value={searchQuery}
                    class="flex-1 bg-transparent border-none text-slate-900 placeholder:text-slate-400 h-12 px-3 text-base focus:ring-0" 
                    placeholder="Search by name (e.g., APA 7th edition)..."
                />
                <button class="hidden sm:flex items-center justify-center rounded-lg bg-primary hover:bg-blue-600 text-white font-bold h-10 px-6 transition-colors shadow-md">
                    Find a Style
                </button>
            </div>
            <div class="flex justify-center gap-4 mt-4 text-sm text-slate-500">
                <span>Popular:</span>
                <button class="hover:text-primary hover:underline border-none bg-transparent cursor-pointer" onclick={() => searchQuery = 'APA'}>APA 7th</button>
                <button class="hover:text-primary hover:underline border-none bg-transparent cursor-pointer" onclick={() => searchQuery = 'MLA'}>MLA 9th</button>
                <button class="hover:text-primary hover:underline border-none bg-transparent cursor-pointer" onclick={() => searchQuery = 'Chicago'}>Chicago</button>
            </div>
        </div>

        <!-- Secondary Action -->
        <div class="mt-4">
            <a class="inline-flex items-center gap-2 text-primary font-bold hover:text-blue-400 transition-colors group" href="/create-wizard">
                <span class="material-symbols-outlined text-xl group-hover:translate-x-1 transition-transform">arrow_forward</span>
                <span>Create a custom style with the wizard</span>
            </a>
        </div>
    </div>
</section>

<!-- Featured Styles Section -->
<section class="py-16 px-4 lg:px-10 bg-white">
    <div class="max-w-[1200px] mx-auto">
        <div class="flex items-center justify-between mb-10">
            <h2 class="text-2xl font-black leading-tight tracking-tight text-slate-900">
                {searchQuery ? `Search Results (${filteredStyles.length})` : 'Trending Standards'}
            </h2>
            <a class="text-sm font-bold text-primary hover:text-blue-400" href="#">View all styles</a>
        </div>
        
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
            {#each filteredStyles as style}
                <a href="/style/{style.id}" class="group relative flex flex-col overflow-hidden rounded-2xl border border-slate-100 bg-slate-50 hover:border-primary/50 hover:shadow-2xl hover:shadow-primary/5 transition-all duration-300">
                    <div class="h-32 bg-gradient-to-r {style.color} relative p-6 flex flex-col justify-end">
                        {#if style.verified}
                            <div class="absolute top-4 right-4">
                                <span class="inline-flex items-center rounded-full bg-white/20 backdrop-blur-sm px-2.5 py-0.5 text-[10px] font-black uppercase tracking-wider text-white ring-1 ring-inset ring-white/30">
                                    Verified
                                </span>
                            </div>
                        {/if}
                        <h3 class="text-2xl font-black text-white tracking-tight">{style.name}</h3>
                        <p class="text-white/80 text-xs font-medium">{style.type}</p>
                    </div>
                    <div class="p-6 flex flex-col gap-4 flex-1">
                        <div class="flex flex-col gap-2">
                            <span class="text-[10px] uppercase font-black tracking-widest text-slate-400">Preview</span>
                            <div class="font-serif text-sm leading-relaxed text-slate-700">
                                {@html previews[style.id] || style.preview}
                            </div>
                        </div>
                        <div class="mt-auto pt-6 flex gap-3 border-t border-slate-100">
                            <button class="flex-1 rounded-lg bg-white border border-slate-200 hover:bg-slate-50 text-slate-900 h-10 text-xs font-bold transition-all">
                                Use this Style
                            </button>
                            <button 
                                onclick={(e) => { e.preventDefault(); toggleBookmark(style.id); }}
                                class="flex items-center justify-center w-10 h-10 rounded-lg border border-slate-200 {$bookmarks.includes(style.id) ? 'text-primary bg-blue-50 border-primary/20' : 'text-slate-400 hover:text-primary'} transition-colors"
                            >
                                <span class="material-symbols-outlined text-[20px] {$bookmarks.includes(style.id) ? 'fill-1' : ''}">
                                    bookmark
                                </span>
                            </button>
                        </div>
                    </div>
                </a>
            {/each}

            {#if filteredStyles.length === 0}
                <div class="col-span-full py-20 text-center">
                    <span class="material-symbols-outlined text-6xl text-slate-200 mb-4">search_off</span>
                    <p class="text-slate-500 font-medium">No styles found matching "{searchQuery}"</p>
                    <button 
                        class="mt-4 text-primary font-bold hover:underline"
                        onclick={() => searchQuery = ''}
                    >
                        Clear search
                    </button>
                </div>
            {/if}
        </div>
    </div>
</section>