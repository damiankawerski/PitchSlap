interface AudioEngineControlsProps {
    onChangeHandler: () => void;
    title: string;
    description: string;
    isRunning: boolean;
    startButtonText: string;
    stopButtonText: string;
}

export default function AudioEngineControls(props: AudioEngineControlsProps) {
    return (
        <div className="relative bg-gradient-to-br from-gray-900/90 via-gray-800/90 to-black/90 rounded-2xl p-6 border border-purple-500/30 shadow-xl shadow-purple-500/10 backdrop-blur-sm">
            {/* Animated background glow */}
            <div className="absolute inset-0 bg-gradient-to-r from-purple-600/5 via-pink-600/5 to-orange-600/5 rounded-2xl blur-xl"></div>
            
            {/* Content */}
            <div className="relative z-10">
                {/* Header with status indicator */}
                <div className="flex items-center justify-between mb-4">
                    <h2 className="text-xl font-bold bg-gradient-to-r from-purple-400 via-pink-400 to-orange-400 bg-clip-text text-transparent">
                        {props.title}
                    </h2>
                    <div className="flex items-center gap-2">
                        <div className={`w-3 h-3 rounded-full ${props.isRunning ? 'bg-green-500 animate-pulse' : 'bg-gray-500'} shadow-lg`}></div>
                        <span className={`text-sm font-medium ${props.isRunning ? 'text-green-400' : 'text-gray-400'}`}>
                            {props.isRunning ? 'Active' : 'Inactive'}
                        </span>
                    </div>
                </div>

                {/* Description */}
                <p className="text-gray-300 text-sm mb-6 leading-relaxed">
                    {props.description}
                </p>

                {/* Action button */}
                <button 
                    onClick={props.onChangeHandler}
                    className={`
                        w-full px-6 py-3 rounded-xl font-semibold text-white transition-all duration-300 transform hover:scale-[1.02] focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-900 shadow-lg
                        ${props.isRunning 
                            ? 'bg-gradient-to-r from-red-600 to-red-700 hover:from-red-500 hover:to-red-600 focus:ring-red-500 shadow-red-500/25' 
                            : 'bg-gradient-to-r from-purple-600 via-pink-600 to-orange-600 hover:from-purple-500 hover:via-pink-500 hover:to-orange-500 focus:ring-purple-500 shadow-purple-500/25'
                        }
                    `}
                >
                    <span className="flex items-center justify-center gap-2">
                        {props.isRunning ? (
                            <>
                                <span className="w-2 h-2 bg-white rounded-full"></span>
                                {props.stopButtonText}
                            </>
                        ) : (
                            <>
                                <span className="w-0 h-0 border-l-[6px] border-l-white border-y-[4px] border-y-transparent"></span>
                                {props.startButtonText}
                            </>
                        )}
                    </span>
                </button>
            </div>

            {/* Decorative corner elements */}
            <div className="absolute top-2 right-2 w-2 h-2 bg-gradient-to-r from-purple-400 to-pink-400 rounded-full opacity-60"></div>
            <div className="absolute bottom-2 left-2 w-1 h-1 bg-gradient-to-r from-pink-400 to-orange-400 rounded-full opacity-40"></div>
        </div>
    );
}