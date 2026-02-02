import React from 'react';
import { MainLayout } from '../layouts/MainLayout';
import { Link } from 'react-router-dom';

export const HomePage: React.FC = () => {
    return (
        <MainLayout>
            <div className="flex flex-col gap-8">
                <div className="text-center py-16">
                    <h1 className="text-4xl font-black text-text-main mb-4 tracking-tight">The Next Generation of CSL Citation Styles</h1>
                    <p className="text-text-secondary text-lg max-w-2xl mx-auto">
                        Find, edit, and create CSL citation styles for your research.
                    </p>
                    <div className="mt-8 flex justify-center gap-4">
                        <Link to="/create-wizard" className="bg-primary text-white px-6 py-3 rounded-xl font-bold hover:bg-blue-700 shadow-lg shadow-blue-500/20 transition-all">
                            Create New Style
                        </Link>
                        <button className="bg-white border border-border-light text-text-main px-6 py-3 rounded-xl font-bold hover:bg-gray-50 transition-colors">
                            Browse Repository
                        </button>
                    </div>
                </div>

                {/* Placeholder for list of styles */}
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    {/* We can map mock data here later */}
                    <div className="bg-white p-6 rounded-xl border border-border-light shadow-sm hover:shadow-md transition-shadow">
                        <h3 className="text-xl font-bold text-text-main mb-2">APA 7th Edition</h3>
                        <p className="text-text-secondary text-sm mb-4">American Psychological Association</p>
                        <div className="flex gap-2">
                            <Link to="/style/apa-7" className="text-primary text-sm font-bold hover:underline">View Details</Link>
                        </div>
                    </div>
                </div>
            </div>
        </MainLayout>
    );
};
