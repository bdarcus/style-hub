import React, { type ReactNode } from 'react';
import { Link, useLocation } from 'react-router-dom';

interface MainLayoutProps {
    children?: ReactNode;
}

export const MainLayout: React.FC<MainLayoutProps> = ({ children }) => {
    const location = useLocation();

    const isActive = (path: string) => {
        return location.pathname === path ? 'text-text-main' : 'text-text-secondary';
    };

    return (
        <div className="bg-background-light dark:bg-background-dark text-text-main antialiased min-h-screen flex flex-col font-display">
            <header className="sticky top-0 z-50 w-full bg-surface-light border-b border-border-light">
                <div className="max-w-[1440px] mx-auto px-4 sm:px-6 lg:px-8">
                    <div className="flex h-16 items-center justify-between">
                        {/* Logo & Nav */}
                        <div className="flex items-center gap-8">
                            <Link to="/" className="flex items-center gap-3">
                                <div className="size-8 text-primary bg-primary/10 rounded-lg flex items-center justify-center">
                                    <span className="material-symbols-outlined text-2xl">school</span>
                                </div>
                                <h2 className="text-text-main text-xl font-bold tracking-tight hidden sm:block">Citation Repo</h2>
                            </Link>
                            <nav className="hidden md:flex items-center gap-6">
                                <Link to="/" className={`text-sm font-medium hover:text-primary transition-colors ${isActive('/')}`}>Styles</Link>
                                <Link to="/editor" className={`text-sm font-medium hover:text-primary transition-colors ${isActive('/editor')}`}>Editor</Link>
                                <Link to="/guide" className={`text-sm font-medium hover:text-primary transition-colors ${isActive('/guide')}`}>Guide</Link>
                                <Link to="/about" className={`text-sm font-medium hover:text-primary transition-colors ${isActive('/about')}`}>About</Link>
                            </nav>
                        </div>
                        {/* Search & Actions */}
                        <div className="flex items-center gap-4">
                            <div className="hidden sm:flex relative group">
                                <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                                    <span className="material-symbols-outlined text-text-secondary text-[20px]">search</span>
                                </div>
                                <input
                                    className="block w-full rounded-lg border-none bg-background-light py-2 pl-10 pr-3 text-sm placeholder:text-text-secondary focus:ring-2 focus:ring-primary focus:bg-white transition-all w-64"
                                    placeholder="Search styles..."
                                    type="text"
                                />
                            </div>
                            <button className="bg-primary text-white text-sm font-bold px-5 py-2 rounded-lg hover:bg-blue-700 transition-colors shadow-sm shadow-blue-200">
                                Log In
                            </button>
                        </div>
                    </div>
                </div>
            </header>
            <main className="flex-1 max-w-[1440px] mx-auto w-full px-4 sm:px-6 lg:px-8 py-8">
                {children}
            </main>
        </div>
    );
};
