import React, { useState } from 'react';
import { MainLayout } from '../layouts/MainLayout';

export const StyleDetailPage: React.FC = () => {
    const [activeTab, setActiveTab] = useState<'citations' | 'bibliography'>('citations');

    return (
        <MainLayout>
            {/* Breadcrumbs */}
            <nav aria-label="Breadcrumb" className="flex mb-8">
                <ol className="inline-flex items-center space-x-1 md:space-x-2 rtl:space-x-reverse">
                    <li className="inline-flex items-center">
                        <a href="/" className="inline-flex items-center text-sm font-medium text-text-secondary hover:text-primary">
                            <span className="material-symbols-outlined text-[18px] mr-1">home</span>
                            Home
                        </a>
                    </li>
                    <li>
                        <div className="flex items-center">
                            <span className="material-symbols-outlined text-text-secondary text-[16px]">chevron_right</span>
                            <a href="#" className="ms-1 text-sm font-medium text-text-secondary hover:text-primary md:ms-2">Styles</a>
                        </div>
                    </li>
                    <li aria-current="page">
                        <div className="flex items-center">
                            <span className="material-symbols-outlined text-text-secondary text-[16px]">chevron_right</span>
                            <span className="ms-1 text-sm font-medium text-text-main md:ms-2">APA 7th Edition</span>
                        </div>
                    </li>
                </ol>
            </nav>

            {/* Two Column Layout */}
            <div className="grid grid-cols-1 lg:grid-cols-12 gap-10 items-start">
                {/* Left Sidebar (Metadata & Actions) */}
                <aside className="lg:col-span-4 flex flex-col gap-6 sticky top-24">
                    {/* Title Block */}
                    <div className="flex flex-col gap-2">
                        <h1 className="text-3xl lg:text-4xl font-black text-text-main leading-tight tracking-tight">
                            APA 7th Edition
                        </h1>
                        <p className="text-text-secondary text-sm font-medium">American Psychological Association</p>
                        <div className="mt-2 inline-flex items-center gap-2 bg-green-50 text-green-700 px-3 py-1 rounded-full w-fit border border-green-100">
                            <span className="material-symbols-outlined text-[16px]">check_circle</span>
                            <span className="text-xs font-bold uppercase tracking-wide">Verified Style</span>
                        </div>
                    </div>
                    {/* Action Buttons */}
                    <div className="flex flex-col gap-3 py-4 border-b border-border-light">
                        <button className="flex w-full items-center justify-center gap-3 bg-primary text-white h-12 rounded-lg font-bold shadow-lg shadow-blue-500/20 hover:bg-blue-700 hover:shadow-blue-500/30 transition-all active:scale-[0.98]">
                            <span className="material-symbols-outlined">download</span>
                            Install Style
                        </button>
                        <div className="flex gap-3">
                            <button className="flex-1 flex items-center justify-center gap-2 bg-white border border-border-light text-text-main h-10 rounded-lg text-sm font-bold hover:bg-gray-50 transition-colors">
                                <span className="material-symbols-outlined text-[20px]">code</span>
                                Source
                            </button>
                            <button className="flex-1 flex items-center justify-center gap-2 bg-white border border-border-light text-text-main h-10 rounded-lg text-sm font-bold hover:bg-gray-50 transition-colors">
                                <span className="material-symbols-outlined text-[20px]">edit</span>
                                Edit
                            </button>
                        </div>
                    </div>
                    {/* Metadata List */}
                    <div className="flex flex-col gap-5 py-2">
                        <h3 className="text-text-main text-sm font-bold uppercase tracking-wider opacity-70">Metadata</h3>
                        <div className="grid grid-cols-1 gap-4">
                            <MetadataItem icon="menu_book" label="Field" value="Social Sciences" />
                            <MetadataItem icon="format_quote" label="Format" value="Author-Date" />
                            <MetadataItem icon="policy" label="License" value="CC-BY-SA 4.0" />
                            <MetadataItem icon="update" label="Last Updated" value="October 24, 2023" />
                        </div>
                        <div className="mt-4 pt-6 border-t border-border-light">
                            <p className="text-sm text-text-secondary leading-relaxed">
                                This style follows the guidelines of the 7th edition of the Publication Manual of the American Psychological Association.
                            </p>
                        </div>
                    </div>
                </aside>

                {/* Right Content (Previews) */}
                <section className="lg:col-span-8 flex flex-col gap-10 min-w-0">
                    {/* Section Header */}
                    <div className="flex items-end justify-between border-b border-border-light pb-4">
                        <div>
                            <h2 className="text-xl font-bold text-text-main">Comprehensive Preview</h2>
                            <p className="text-sm text-text-secondary mt-1">Review how this style handles common citation scenarios.</p>
                        </div>
                        <div className="flex gap-2 bg-white rounded-lg p-1 border border-border-light shadow-sm">
                            <button
                                onClick={() => setActiveTab('citations')}
                                className={`px-3 py-1.5 rounded text-xs font-bold transition-colors ${activeTab === 'citations' ? 'bg-primary/10 text-primary' : 'text-text-secondary hover:bg-background-light'}`}
                            >
                                Citations
                            </button>
                            <button
                                onClick={() => setActiveTab('bibliography')}
                                className={`px-3 py-1.5 rounded text-xs font-bold transition-colors ${activeTab === 'bibliography' ? 'bg-primary/10 text-primary' : 'text-text-secondary hover:bg-background-light'}`}
                            >
                                Bibliography
                            </button>
                        </div>
                    </div>

                    {/* In-Text Citations Block */}
                    <div className="flex flex-col gap-4 group/section">
                        <div className="flex items-center gap-2">
                            <span className="material-symbols-outlined text-primary text-[20px]">short_text</span>
                            <h3 className="text-base font-bold text-text-main">In-Text Citations</h3>
                        </div>
                        <div className="bg-white rounded-xl shadow-sm border border-border-light overflow-hidden">
                            <CitationPreviewItem label="Single" content="(Smith, 2023)" />
                            <CitationPreviewItem label="Multiple" content="(Jones, 2021; Smith, 2023)" />
                            <CitationPreviewItem label="With Page" content="(Smith, 2023, p. 45)" />
                            <CitationPreviewItem label="Narrative" content="According to Smith (2023)..." />
                        </div>
                    </div>

                    {/* Bibliography Block */}
                    <div className="flex flex-col gap-4">
                        <div className="flex items-center gap-2">
                            <span className="material-symbols-outlined text-primary text-[20px]">list_alt</span>
                            <h3 className="text-base font-bold text-text-main">Bibliography Preview</h3>
                        </div>
                        <div className="bg-white rounded-xl shadow-sm border border-border-light p-8 font-serif">
                            <div className="flex flex-col gap-6">
                                <BibEntry
                                    type="Book"
                                    content={<span>Smith, J. (2023). <i className="italic">The future of artificial intelligence in academia</i>. Academic Press. https://doi.org/10.1038/s41586-023-00000</span>}
                                />
                                <BibEntry
                                    type="Journal Article"
                                    color="purple"
                                    content={<span>Jones, A., & Williams, B. (2021). Citation metrics in the modern era: A comprehensive review. <i className="italic">Journal of Bibliometrics and Information Science</i>, <i className="italic">12</i>(3), 45-60.</span>}
                                />
                                <BibEntry
                                    type="Report"
                                    color="orange"
                                    content={<span>National Institute of Mental Health. (2022). <i className="italic">Clinical training in serious mental illness</i> (DHHS Publication No. ADM 90-1679). U.S. Government Printing Office.</span>}
                                />
                                <BibEntry
                                    type="Webpage"
                                    color="teal"
                                    content={<span>Bologna, C. (2019, October 31). <i className="italic">Why some people with anxiety love watching horror movies</i>. HuffPost. https://www.huffpost.com/entry/anxiety-love-watching-horror-movies_l_5d277587e4b02a5a5d57b59e</span>}
                                />
                            </div>
                        </div>
                    </div>
                </section>
            </div>
        </MainLayout>
    );
};

// Helper Components

const MetadataItem = ({ icon, label, value }: { icon: string, label: string, value: string }) => (
    <div className="flex items-start gap-4">
        <div className="size-10 rounded-lg bg-white border border-border-light flex items-center justify-center text-text-main shrink-0">
            <span className="material-symbols-outlined">{icon}</span>
        </div>
        <div>
            <p className="text-xs text-text-secondary font-medium">{label}</p>
            <p className="text-base text-text-main font-semibold">{value}</p>
        </div>
    </div>
);

const CitationPreviewItem = ({ label, content }: { label: string, content: string }) => (
    <div className="flex flex-col sm:flex-row border-b border-border-light last:border-0 hover:bg-slate-50 transition-colors">
        <div className="w-full sm:w-40 bg-background-light p-4 flex items-center border-b sm:border-b-0 sm:border-r border-border-light">
            <span className="text-xs font-bold text-text-secondary uppercase tracking-wider">{label}</span>
        </div>
        <div className="flex-1 p-5 flex justify-between items-center gap-4">
            <p className="citation-preview text-lg text-slate-800 font-serif">
                {content}
            </p>
            <button aria-label="Copy" className="text-slate-400 hover:text-primary transition-colors p-2 rounded-lg hover:bg-blue-50">
                <span className="material-symbols-outlined text-[20px]">content_copy</span>
            </button>
        </div>
    </div>
);

const BibEntry = ({ type, content, color = 'blue' }: { type: string, content: React.ReactNode, color?: string }) => {
    // Map simple color names to tailwind classes if needed, or use dynamic template literals if safe
    const colorClasses: { [key: string]: { bg: string, text: string } } = {
        blue: { bg: 'bg-blue-50', text: 'text-blue-700' },
        purple: { bg: 'bg-purple-50', text: 'text-purple-700' },
        orange: { bg: 'bg-orange-50', text: 'text-orange-700' },
        teal: { bg: 'bg-teal-50', text: 'text-teal-700' },
    };
    const { bg, text } = colorClasses[color] || colorClasses['blue'];

    return (
        <div className="group relative pl-4 border-l-2 border-transparent hover:border-primary transition-all">
            <div className="absolute -left-[25px] top-1 opacity-0 group-hover:opacity-100 transition-opacity">
                <span className="material-symbols-outlined text-primary text-sm">bookmark</span>
            </div>
            <div className="mb-1 flex items-center gap-2">
                <span className={`${bg} ${text} text-[10px] font-bold px-2 py-0.5 rounded uppercase tracking-wide`}>{type}</span>
            </div>
            <p className="citation-preview text-lg text-slate-900 leading-relaxed pl-8 -indent-8">
                {content}
            </p>
        </div>
    );
};
