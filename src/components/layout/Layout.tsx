
import { Route, Routes } from 'react-router-dom';
import NavBar from './Navbar';
import SettingsPage from '../pages/settings/SettingsPage';
import HomePage from '../pages/Home/HomePage';
import EffectsPage from '../pages/effects/EffectsPage';

export default function Layout() {
    return (
        <div className="flex min-h-screen bg-gray-900">
            <NavBar />
            <main className="flex-1 ml-16 lg:ml-72 p-8 bg-gradient-to-br from-gray-900 via-gray-800 to-black transition-all duration-300">
                <Routes>
                    <Route path="/" element={
                        <HomePage />
                    } />
                    <Route path="/settings" element={
                        <SettingsPage />
                    } />
                    <Route path="/effects" element={
                        <EffectsPage />
                    } />
                </Routes>
            </main>
        </div>
    )
}