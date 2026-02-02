import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { HomePage } from './pages/HomePage';
import { StyleDetailPage } from './pages/StyleDetailPage';
import { WizardPage } from './pages/WizardPage';
import { AboutPage } from './pages/AboutPage';

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<HomePage />} />
        <Route path="/style/:id" element={<StyleDetailPage />} />
        <Route path="/create-wizard" element={<WizardPage />} />
        <Route path="/about" element={<AboutPage />} />
      </Routes>
    </BrowserRouter>
  )
}

export default App
