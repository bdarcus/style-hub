import React, { useState } from 'react';
import { MainLayout } from '../layouts/MainLayout';
import { Link } from 'react-router-dom';

export const WizardPage: React.FC = () => {
    const [step, setStep] = useState(1);
    const [system, setSystem] = useState<'author-date' | 'numeric'>('author-date');
    const [italicizeEtAl, setItalicizeEtAl] = useState(true);
    const [authorLimit, setAuthorLimit] = useState(3);

    return (
        <MainLayout>
            <div className="flex flex-col lg:flex-row gap-10 min-h-[calc(100vh-12rem)] relative">
                {/* LEFT: Configuration Wizard */}
                <aside className="w-full lg:w-[450px] flex flex-col bg-white border border-border-light rounded-2xl shadow-sm overflow-hidden sticky top-24 h-fit">
                    <div className="p-8">
                        {/* Breadcrumbs */}
                        <nav className="flex gap-2 mb-6">
                            <Link to="/" className="text-text-secondary text-xs font-medium hover:underline">All Styles</Link>
                            <span className="text-text-secondary text-xs">/</span>
                            <span className="text-text-main text-xs font-bold">Create New</span>
                        </nav>

                        {/* Title & Progress */}
                        <div className="flex flex-col gap-2 mb-8">
                            <h1 className="text-2xl font-black text-text-main tracking-tight">Create New Style</h1>
                            <p className="text-text-secondary text-sm">Step {step} of 4: Configure basic rules</p>

                            <div className="mt-4 flex flex-col gap-2">
                                <div className="flex justify-between items-end">
                                    <span className="text-[10px] font-bold uppercase tracking-widest text-text-secondary">Progress</span>
                                    <span className="text-primary text-xs font-bold">{step * 25}%</span>
                                </div>
                                <div className="h-1.5 w-full bg-slate-100 rounded-full overflow-hidden">
                                    <div
                                        className="h-full bg-primary transition-all duration-500 ease-out"
                                        style={{ width: `${step * 25}%` }}
                                    />
                                </div>
                            </div>
                        </div>

                        {/* Form Area */}
                        <div className="flex flex-col gap-8">
                            {/* Section 1: Citation System */}
                            <div className="flex flex-col gap-4">
                                <div className="flex items-center gap-2">
                                    <span className="material-symbols-outlined text-primary text-xl">account_tree</span>
                                    <h3 className="text-text-main text-base font-bold">Citation System</h3>
                                </div>
                                <div className="grid grid-cols-1 gap-3">
                                    <SelectionCard
                                        active={system === 'author-date'}
                                        onClick={() => setSystem('author-date')}
                                        title="Author-Date"
                                        description="Citations appear as (Smith, 2023) in the text."
                                    />
                                    <SelectionCard
                                        active={system === 'numeric'}
                                        onClick={() => setSystem('numeric')}
                                        title="Numeric"
                                        description="Citations appear as [1] or ¹ in the text."
                                    />
                                </div>
                            </div>

                            <div className="h-px bg-border-light w-full"></div>

                            {/* Section 2: In-Text Formatting */}
                            <div className="flex flex-col gap-4">
                                <div className="flex items-center gap-2">
                                    <span className="material-symbols-outlined text-primary text-xl">text_fields</span>
                                    <h3 className="text-text-main text-base font-bold">In-Text Formatting</h3>
                                </div>
                                <div className="flex flex-col gap-4">
                                    <ToggleItem
                                        label="Italicize 'et al.'"
                                        checked={italicizeEtAl}
                                        onChange={setItalicizeEtAl}
                                    />
                                    <div className="flex justify-between items-center p-3 rounded-xl bg-slate-50 border border-transparent hover:border-border-light transition-all">
                                        <span className="text-text-main text-sm font-medium">Author Limit before "et al."</span>
                                        <input
                                            type="number"
                                            value={authorLimit}
                                            onChange={(e) => setAuthorLimit(parseInt(e.target.value))}
                                            className="w-16 bg-white border border-border-light rounded-lg p-2 text-center text-sm font-bold focus:ring-2 focus:ring-primary focus:border-transparent outline-none"
                                        />
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>

                    {/* Footer Actions */}
                    <div className="p-6 bg-slate-50 border-t border-border-light mt-auto">
                        <div className="flex gap-4">
                            <button
                                onClick={() => setStep(Math.max(1, step - 1))}
                                className="flex-1 h-12 rounded-xl border border-border-light bg-white text-text-main text-sm font-bold hover:bg-gray-50 transition-colors"
                            >
                                Back
                            </button>
                            <button
                                onClick={() => setStep(Math.min(4, step + 1))}
                                className="flex-[2] h-12 rounded-xl bg-primary text-white text-sm font-bold hover:bg-blue-700 transition-all shadow-lg shadow-blue-500/20 active:scale-95"
                            >
                                Next Step
                            </button>
                        </div>
                    </div>
                </aside>

                {/* RIGHT: Live Preview */}
                <main className="flex-1 bg-slate-100/50 rounded-3xl border border-border-light p-8 lg:p-12 relative overflow-hidden">
                    {/* Preview Document */}
                    <div className="bg-white mx-auto w-full max-w-[700px] min-h-[900px] shadow-2xl p-12 lg:p-16 relative origin-top transition-transform duration-500" style={{ fontFamily: 'Merriweather, serif' }}>
                        <div className="absolute top-0 right-0 bg-slate-100 text-slate-400 px-4 py-1.5 text-[10px] font-sans font-black uppercase tracking-widest rounded-bl-xl">
                            Live Preview
                        </div>

                        <div className="border-b-2 border-slate-900 pb-6 mb-10">
                            <h2 className="text-3xl font-black text-slate-900 leading-tight mb-4">
                                The Evolution of Bibliographic Standards in the Digital Age
                            </h2>
                            <div className="text-slate-600 italic">By Jonathan K. Smith and Maria Rodriguez</div>
                        </div>

                        <div className="flex flex-col gap-6 text-lg leading-relaxed text-slate-800 text-justify mb-16">
                            <p>
                                The rapid proliferation of digital publication platforms has necessitated a re-evaluation of traditional citation metrics.
                                As transitions in scholarship occur, the role of metadata remains central
                                <span className="bg-primary/5 text-primary px-1 rounded-md font-bold mx-1 cursor-help group relative">
                                    {system === 'author-date' ? '(Smith' + (italicizeEtAl ? ' et al.' : ' et al.') + ', 2023)' : '[1]'}
                                    <span className="absolute bottom-full left-1/2 -translate-x-1/2 mb-2 w-48 p-2 bg-slate-900 text-white text-[10px] rounded opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none">
                                        Currently showing {system} style
                                    </span>
                                </span>.
                            </p>
                            <p>
                                Furthermore, studies indicate that consistency in tagging improves discoverability. We propose a framework that adapts contextually based on the medium.
                            </p>
                        </div>

                        <div className="mt-12 pt-8 border-t border-slate-200">
                            <h3 className="text-sm font-black uppercase tracking-[0.2em] mb-8 text-slate-900 font-sans">References</h3>
                            <div className="flex flex-col gap-6">
                                <div className="text-sm text-slate-800 leading-relaxed hanging-indent">
                                    <span className="font-bold">Smith, J. K., Rodriguez, M., & Wu, T. (2023).</span> The end of the footnote? <span className="italic">Academic Publishing Quarterly</span>, 8(1), 112-128. https://doi.org/10.1007/apq.2023.004
                                </div>
                            </div>
                        </div>
                    </div>

                    {/* Floating Zoom Controls */}
                    <div className="absolute bottom-10 left-1/2 -translate-x-1/2 flex items-center gap-2 p-1 bg-white/80 backdrop-blur shadow-xl rounded-full border border-border-light">
                        <button className="size-10 rounded-full flex items-center justify-center text-text-secondary hover:bg-slate-100 transition-colors">
                            <span className="material-symbols-outlined">zoom_out</span>
                        </button>
                        <span className="text-xs font-bold text-text-main px-2">100%</span>
                        <button className="size-10 rounded-full flex items-center justify-center text-text-secondary hover:bg-slate-100 transition-colors">
                            <span className="material-symbols-outlined">zoom_in</span>
                        </button>
                    </div>
                </main>
            </div>
        </MainLayout>
    );
};

// Sub-components

const SelectionCard = ({ active, onClick, title, description }: { active: boolean, onClick: () => void, title: string, description: string }) => (
    <button
        onClick={onClick}
        className={`p-5 rounded-2xl border text-left transition-all ${active ? 'border-primary bg-primary/5 ring-4 ring-primary/5' : 'border-border-light hover:border-text-secondary/30 bg-white'}`}
    >
        <div className="flex items-start gap-3">
            <div className={`mt-0.5 ${active ? 'text-primary' : 'text-slate-300'}`}>
                <span className="material-symbols-outlined">{active ? 'radio_button_checked' : 'radio_button_unchecked'}</span>
            </div>
            <div>
                <span className="block text-text-main text-sm font-bold mb-1">{title}</span>
                <span className="block text-text-secondary text-xs leading-relaxed">{description}</span>
            </div>
        </div>
    </button>
);

const ToggleItem = ({ label, checked, onChange }: { label: string, checked: boolean, onChange: (v: boolean) => void }) => (
    <label className="flex items-center justify-between p-4 rounded-xl bg-slate-50 cursor-pointer hover:bg-slate-100 transition-colors group">
        <span className="text-text-main text-sm font-medium">{label}</span>
        <div className="relative inline-flex items-center cursor-pointer" onClick={() => onChange(!checked)}>
            <div className={`w-11 h-6 rounded-full transition-colors ${checked ? 'bg-primary' : 'bg-slate-300'}`}>
                <div className={`absolute top-[2px] left-[2px] bg-white border border-slate-300 rounded-full h-5 w-5 transition-transform ${checked ? 'translate-x-5' : ''}`}></div>
            </div>
        </div>
    </label>
);
