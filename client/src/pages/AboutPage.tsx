import React from 'react';
import { MainLayout } from '../layouts/MainLayout';

export const AboutPage: React.FC = () => {
    return (
        <MainLayout>
            <div className="max-w-3xl mx-auto py-12">
                <div className="mb-12">
                    <h1 className="text-4xl font-black text-text-main tracking-tight mb-6">About Citation Style Editor</h1>
                    <p className="text-xl text-text-secondary leading-relaxed font-medium">
                        A modern, open-source platform for managing and creating Citation Style Language (CSL) files.
                    </p>
                </div>

                <div className="grid gap-10">
                    <section className="flex flex-col gap-4">
                        <div className="flex items-center gap-3">
                            <div className="size-10 rounded-xl bg-primary/10 flex items-center justify-center text-primary">
                                <span className="material-symbols-outlined">auto_awesome</span>
                            </div>
                            <h2 className="text-2xl font-bold text-text-main">The Mission</h2>
                        </div>
                        <p className="text-text-main leading-7 opacity-80">
                            Our goal is to lower the barrier to entry for academic bibliography management. Citation styles are notoriously complex, and CSL—while powerful—is often difficult for researchers to edit without deep technical knowledge.
                        </p>
                    </section>

                    <section className="flex flex-col gap-4">
                        <div className="flex items-center gap-3">
                            <div className="size-10 rounded-xl bg-orange-100 flex items-center justify-center text-orange-600">
                                <span className="material-symbols-outlined">terminal</span>
                            </div>
                            <h2 className="text-2xl font-bold text-text-main">Tech Stack</h2>
                        </div>
                        <ul className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                            <TechCard
                                title="Rust Backend"
                                description="High-performance CSL processing engine."
                                icon="settings"
                            />
                            <TechCard
                                title="React + Vite"
                                description="Lightning-fast frontend development and UX."
                                icon="bolt"
                            />
                            <TechCard
                                title="Tailwind CSS v4"
                                description="Modern, design-token based utility styling."
                                icon="palette"
                            />
                            <TechCard
                                title="CSL Standard"
                                description="Full compatibility with citation-style-language."
                                icon="description"
                            />
                        </ul>
                    </section>
                </div>
            </div>
        </MainLayout>
    );
};

const TechCard = ({ title, description, icon }: { title: string, description: string, icon: string }) => (
    <li className="p-5 rounded-2xl bg-white border border-border-light shadow-sm hover:shadow-md transition-shadow flex gap-4">
        <span className="material-symbols-outlined text-text-secondary mt-0.5">{icon}</span>
        <div>
            <h3 className="font-bold text-text-main text-sm">{title}</h3>
            <p className="text-text-secondary text-xs mt-1 leading-relaxed">{description}</p>
        </div>
    </li>
);
