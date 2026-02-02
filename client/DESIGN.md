# Design System: Citation Style Detailed Preview
**Project ID:** projects/4997510721725575250

## 1. Visual Theme & Atmosphere
The design presents a clean, academic, and professional atmosphere ("Clean Academic"). It uses ample whitespace, a refined color palette, and high legibility typography (Lexend and Merriweather) to create a trustworthy environment for managing citations. The layout is structured with a clearly defined sidebar and main content area, typical of documentation or dashboard interfaces.

## 2. Color Palette & Roles
* **Primary Blue (#135bec):** Used for primary actions (buttons), brand icons, and active states.
* **Background Light (#f8f9fc):** Used for the main page background, creating a subtle contrast with white surfaces.
* **Background Dark (#101622):** (Inferred) Dark mode background.
* **Surface White (#ffffff):** Used for cards, the header, and content containers.
* **Border Light (#e7ebf3):** Used for subtle dividers and borders to separate content areas.
* **Text Main (#0d121b):** Used for headings, body text, and primary information.
* **Text Secondary (#4c669a):** Used for metadata, labels, and secondary supporting text.

## 3. Typography Rules
* **Font Family (Display/UI):** Lexend (Sans-serif). Used for specific UI elements, headings, and navigation.
* **Font Family (Serif):** Merriweather (Serif). Used for citation previews to mimic academic publishing standards.
* **Weights:**
    *   **Bold (700/800):** Headings, buttons.
    *   **Medium (500):** Navigation links, metadata labels.
    *   **Regular (400):** Body text.

## 4. Component Stylings
* **Buttons:**
    *   **Primary:** `rounded-lg`, `bg-primary`, `text-white`, `shadow-lg`, `shadow-blue-500/20`.
    *   **Secondary/Outline:** `rounded-lg`, `bg-white`, `border border-border-light`, `text-text-main`.
    *   **Icon-only:** `rounded-lg`, `hover:bg-blue-50`.
* **Cards/Containers:**
    *   `rounded-xl`, `bg-white`, `shadow-sm`, `border border-border-light`.
* **Inputs/Forms:**
    *   Search bar: `rounded-lg`, `bg-background-light`, `border-none`.
* **Navigation:**
    *   Sidebar link: `hover:text-primary`, `text-sm`, `font-medium`.
    *   Breadcrumbs: `text-text-secondary`, `hover:text-primary`.

## 5. Layout Principles
* **Structure:** Top navigation bar (`sticky`), split layout with sidebar (`lg:col-span-4`) and main content (`lg:col-span-8`).
* **Spacing:** Generous padding (`p-8`, `gap-10`) to separate semantic sections.
* **Grid:** 12-column grid used for the main layout structure.
