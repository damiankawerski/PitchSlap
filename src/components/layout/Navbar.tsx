import {NavLink} from "react-router-dom";

const navItems = [
    { name: "🏠 Home", path: "/", icon: "🏠" },
    { name: "⚙️ Settings", path: "/settings", icon: "⚙️" },
    { name: "🎵 Effects", path: "/effects", icon: "🎵" },
]

export default function NavBar() {
    return (
        <nav className="bg-gradient-to-b from-gray-900 via-gray-800 to-black text-white fixed left-0 top-0 h-full w-16 lg:w-72 flex flex-col shadow-2xl border-r border-purple-500/20 backdrop-blur-sm transition-all duration-300">
            {/* Header with animated glow */}
            <div className="relative p-3 lg:p-8 border-b border-gray-700/50">
                <div className="absolute inset-0 bg-gradient-to-r from-purple-600/10 via-pink-600/10 to-orange-600/10 blur-xl"></div>
                <h1 className="relative text-xl lg:text-4xl font-black tracking-wider bg-gradient-to-r from-purple-400 via-pink-400 to-orange-400 bg-clip-text text-transparent drop-shadow-2xl text-center">
                    <span className="lg:inline hidden">PitchSlap</span>
                    <span className="lg:hidden inline">PS</span>
                </h1>
                <div className="mt-2 h-1 bg-gradient-to-r from-purple-500 via-pink-500 to-orange-500 rounded-full animate-pulse"></div>
            </div>

            {/* Navigation items */}
            <div className="flex-1 p-2 lg:p-6 space-y-1 lg:space-y-3">
                {navItems.map((item, index) => (
                    <div key={item.path} className="relative group">
                        <NavLink
                            to={item.path}
                            className={({ isActive }) =>
                                isActive 
                                ? "relative flex items-center justify-center lg:justify-start px-2 lg:px-6 py-2 lg:py-4 text-sm lg:text-lg font-semibold text-white bg-gradient-to-r from-purple-600/20 via-pink-600/20 to-orange-600/20 rounded-xl border border-purple-500/30 shadow-lg shadow-purple-500/10 transform transition-all duration-300" 
                                : "relative flex items-center justify-center lg:justify-start px-2 lg:px-6 py-2 lg:py-4 text-sm lg:text-lg font-medium text-gray-300 hover:text-white rounded-xl hover:bg-gradient-to-r hover:from-gray-800/50 hover:to-gray-700/50 border border-transparent hover:border-gray-600/30 transition-all duration-300 hover:transform hover:scale-[1.02] hover:shadow-lg"
                            }
                            title={item.name.split(' ').slice(1).join(' ')} // Tooltip na małych ekranach
                        >
                            {({ isActive }) => (
                                <>
                                    {isActive && (
                                        <div className="absolute inset-0 bg-gradient-to-r from-purple-600/10 via-pink-600/10 to-orange-600/10 rounded-xl blur-sm"></div>
                                    )}
                                    <span className="relative z-10 flex items-center gap-0 lg:gap-3">
                                        <span className="text-base lg:text-xl">{item.icon}</span>
                                        <span className="tracking-wide lg:inline hidden">{item.name.split(' ').slice(1).join(' ')}</span>
                                    </span>
                                    {isActive && (
                                        <div className="absolute right-1 lg:right-4 w-1.5 h-1.5 lg:w-2 lg:h-2 bg-gradient-to-r from-purple-400 to-pink-400 rounded-full animate-pulse"></div>
                                    )}
                                </>
                            )}
                        </NavLink>
                    </div>
                ))}
            </div>

            {/* Bottom decorative element */}
            <div className="p-2 lg:p-6 border-t border-gray-700/50">
                <div className="flex items-center justify-center space-x-1 lg:space-x-2">
                    <div className="w-1 h-1 lg:w-2 lg:h-2 bg-purple-500 rounded-full animate-pulse"></div>
                    <div className="w-1 h-1 lg:w-2 lg:h-2 bg-pink-500 rounded-full animate-pulse" style={{animationDelay: '0.5s'}}></div>
                    <div className="w-1 h-1 lg:w-2 lg:h-2 bg-orange-500 rounded-full animate-pulse" style={{animationDelay: '1s'}}></div>
                </div>
            </div>
        </nav>
    );
}