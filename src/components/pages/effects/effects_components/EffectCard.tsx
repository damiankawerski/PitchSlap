interface EffectCardProps {
    title: string;
    onChangeHandler: (name: string) => void;
    image: string;
    isActive: boolean;
    slug: string;
}

export default function EffectCard({ title, onChangeHandler, image, isActive, slug }: EffectCardProps) {
    return (
        <div 
            onClick={() => onChangeHandler(slug)}
            className="relative group cursor-pointer transform transition-all duration-300 hover:scale-[1.02] active:scale-[0.98] w-full max-w-[300px] mx-auto"
        >
            {/* Animated rainbow border when active */}
            {isActive && (
                <div className="absolute -inset-1 bg-gradient-to-r from-purple-600 via-pink-600 via-red-500 via-orange-500 via-yellow-500 via-green-500 via-blue-500 to-purple-600 rounded-2xl blur-sm opacity-70 animate-pulse">
                    <div className="absolute inset-0 bg-gradient-to-r from-purple-600 via-pink-600 via-red-500 via-orange-500 via-yellow-500 via-green-500 via-blue-500 to-purple-600 rounded-2xl animate-spin-slow"></div>
                </div>
            )}

            {/* Hover glow effect */}
            <div className={`absolute -inset-0.5 bg-gradient-to-r from-purple-600/20 via-pink-600/20 to-orange-600/20 rounded-2xl blur opacity-0 group-hover:opacity-100 transition duration-500 ${isActive ? 'opacity-30' : ''}`}></div>
            
            {/* Main card - square aspect ratio with fixed max size */}
            <div className={`relative bg-gradient-to-br from-gray-900/95 via-gray-800/95 to-black/95 rounded-2xl overflow-hidden border backdrop-blur-sm transition-all duration-300 aspect-square max-h-[300px] ${
                isActive 
                    ? 'border-transparent shadow-xl shadow-purple-500/20' 
                    : 'border-purple-500/30 hover:border-purple-500/50'
            }`}>
                
                {/* Square image container */}
                <div className="relative h-2/3 overflow-hidden">
                    {/* Background image */}
                    <img 
                        src={image} 
                        alt={title}
                        className="w-full h-full object-cover transition-transform duration-500 group-hover:scale-110"
                    />
                    
                    {/* Gradient overlay */}
                    <div className="absolute inset-0 bg-gradient-to-t from-black/80 via-black/20 to-transparent"></div>
                    
                    {/* Active indicator */}
                    {isActive && (
                        <div className="absolute top-2 right-2 bg-gradient-to-r from-green-500 to-emerald-500 text-white text-xs font-bold px-1.5 py-0.5 rounded-full shadow-lg animate-bounce">
                            ACTIVE
                        </div>
                    )}

                    {/* Floating particles when active */}
                </div>

                {/* Content - takes bottom 1/3 */}
                <div className="p-2.5 h-1/3 flex flex-col justify-center">
                    {/* Title */}
                    <h3 className={`font-bold text-sm transition-all duration-300 text-center leading-tight ${
                        isActive 
                            ? 'bg-gradient-to-r from-purple-400 via-pink-400 to-orange-400 bg-clip-text text-transparent' 
                            : 'text-white group-hover:text-purple-300'
                    }`}>
                        {title}
                    </h3>

                    {/* Status bar - centered */}
                    <div className="mt-1 flex items-center justify-center gap-1">
                        <div className={`w-1 h-1 rounded-full transition-all duration-300 ${
                            isActive ? 'bg-green-500 animate-pulse' : 'bg-gray-500'
                        }`}></div>
                        <span className={`text-xs font-medium transition-colors duration-300 ${
                            isActive ? 'text-green-400' : 'text-gray-400'
                        }`}>
                            {isActive ? 'Running' : 'Stopped'}
                        </span>
                    </div>

                    {/* Action hint - only visible on hover */}
                    <div className="flex items-center justify-center gap-0.5 opacity-0 group-hover:opacity-100 transition-opacity duration-300 mt-0.5">
                        <div className="w-0.5 h-0.5 bg-purple-400 rounded-full animate-pulse"></div>
                        <span className="text-xs text-purple-400">Click to {isActive ? 'disable' : 'enable'}</span>
                    </div>
                </div>

                {/* Ripple effect on click */}
                <div className="absolute inset-0 overflow-hidden rounded-2xl pointer-events-none">
                    <div className="absolute top-1/2 left-1/2 w-0 h-0 bg-white/20 rounded-full transform -translate-x-1/2 -translate-y-1/2 group-active:w-full group-active:h-full group-active:animate-ping"></div>
                </div>

                {/* Bottom glow when active */}
                {isActive && (
                    <div className="absolute bottom-0 left-0 right-0 h-0.5 bg-gradient-to-r from-purple-500 via-pink-500 to-orange-500 animate-pulse"></div>
                )}
            </div>

            {/* Corner decorations */}
            <div className="absolute top-1 left-1 w-0.5 h-0.5 bg-gradient-to-r from-purple-400 to-pink-400 rounded-full opacity-60"></div>
            <div className="absolute bottom-1 right-1 w-0.5 h-0.5 bg-gradient-to-r from-pink-400 to-orange-400 rounded-full opacity-40"></div>
        </div>
    );
}