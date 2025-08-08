import OptionDropdown from "./OptionDropdown";

interface OptionCardProps {
    currentOption: string;
    availableOptions: string[];
    onOptionChange: (option: string) => void;
    title: string;
    description: string;
    icon: React.ReactNode;
}

export default function OptionCard({ 
    currentOption, 
    availableOptions, 
    onOptionChange, 
    title, 
    description, 
    icon 
}: OptionCardProps) {
    const rainbowGradient = "bg-gradient-to-r from-red-500 via-yellow-500 via-green-500 via-blue-500 via-indigo-500 to-purple-500";
    
    return (
        <div className="bg-gradient-to-br from-gray-900/50 via-gray-800/30 to-black/50 backdrop-blur-sm border border-gray-700/30 rounded-2xl p-6 shadow-2xl">
            {/* Header */}
            <div className="flex items-center justify-center gap-4 mb-6">
                <div className="relative p-4 rounded-xl border border-white/10 shadow-xl">
                    <div className={`absolute inset-0 ${rainbowGradient} rounded-xl animate-pulse opacity-80`}></div>
                    <div className={`absolute inset-0 ${rainbowGradient} rounded-xl blur-lg opacity-30`}></div>
                    <div className="relative z-10 w-8 h-8 text-white drop-shadow-lg">
                        {icon}
                    </div>
                </div>
                <div className="text-center">
                    <h3 className={`text-2xl font-bold ${rainbowGradient} bg-clip-text text-transparent animate-pulse mb-2`}>
                        {title}
                    </h3>
                    <p className="text-gray-300 text-base">
                        {description}
                    </p>
                </div>
            </div>

            <OptionDropdown
                currentOption={currentOption}
                availableOptions={availableOptions}
                onOptionChange={onOptionChange}
            />
        </div>
    );
}