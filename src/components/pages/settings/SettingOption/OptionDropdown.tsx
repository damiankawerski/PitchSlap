

import { useState } from 'react';

interface OptionDropDownProps {
    currentOption: string;
    availableOptions: string[];
    onOptionChange: (option: string) => void;
}

export default function OptionDropDown({ currentOption, availableOptions, onOptionChange }: OptionDropDownProps) {
    const [isOpen, setIsOpen] = useState(false);

    const handleOptionSelect = (option: string) => {
        onOptionChange(option);
        setIsOpen(false);
    };

    const toggleDropdown = () => {
        setIsOpen(!isOpen);
    };

    return (
        <div className="relative">
             {/* Status indicator */}
            <div className="mt-2 flex items-center justify-between text-xs text-gray-500">
                <span>{availableOptions.length} options available</span>
            </div>
            {/* Dropdown Header */}
            <div 
                onClick={toggleDropdown}
                className="bg-gray-800 border border-gray-700 rounded px-4 py-4 cursor-pointer text-white w-full text-center font-bold text-lg hover:bg-gray-700 transition-all duration-200 flex items-center justify-between"
            >
                <span className="flex-1">{currentOption}</span>
                <div className="flex items-center gap-2">
                    {/* Dropdown Arrow */}
                    <svg 
                        className={`w-5 h-5 transition-transform duration-200 ${isOpen ? 'rotate-180' : ''}`} 
                        fill="currentColor" 
                        viewBox="0 0 20 20"
                    >
                        <path fillRule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clipRule="evenodd" />
                    </svg>
                </div>
            </div>

            {/* Dropdown Options */}
            <div className={`transition-all duration-300 ease-in-out overflow-hidden ${
                isOpen ? 'max-h-96 opacity-100' : 'max-h-0 opacity-0'
            }`}>
                <div 
                    className="mt-1 bg-gray-800 border border-gray-700 rounded shadow-2xl max-h-80 overflow-y-auto custom-scrollbar"
                    style={{
                        scrollbarWidth: 'thin',
                        scrollbarColor: '#8b5cf6 #374151'
                    }}
                >
                    <style dangerouslySetInnerHTML={{
                        __html: `
                            .custom-scrollbar::-webkit-scrollbar {
                                width: 8px;
                            }
                            .custom-scrollbar::-webkit-scrollbar-track {
                                background: rgba(55, 65, 81, 0.5);
                                border-radius: 4px;
                                margin: 4px;
                            }
                            .custom-scrollbar::-webkit-scrollbar-thumb {
                                background: linear-gradient(45deg, #8b5cf6, #ec4899);
                                border-radius: 4px;
                                border: 1px solid rgba(75, 85, 99, 0.3);
                            }
                            .custom-scrollbar::-webkit-scrollbar-thumb:hover {
                                background: linear-gradient(45deg, #7c3aed, #db2777);
                                box-shadow: 0 0 8px rgba(139, 92, 246, 0.3);
                            }
                            .custom-scrollbar::-webkit-scrollbar-thumb:active {
                                background: linear-gradient(45deg, #6d28d9, #be185d);
                            }
                            .custom-scrollbar::-webkit-scrollbar-corner {
                                background: transparent;
                            }
                        `
                    }} />
                    {availableOptions.map((option, index) => (
                        <div
                            key={option}
                            onClick={() => handleOptionSelect(option)}
                            className={`px-4 py-3 cursor-pointer font-bold text-lg transition-all duration-200 border-b border-gray-700/50 last:border-b-0 transform ${
                                isOpen ? 'translate-y-0 opacity-100' : '-translate-y-2 opacity-0'
                            } ${
                                option === currentOption
                                    ? 'bg-gradient-to-r from-purple-600/20 to-pink-600/20 text-white border-l-4 border-purple-500'
                                    : 'text-gray-300 hover:text-white hover:bg-gray-700/50'
                            }`}
                            style={{
                                transitionDelay: isOpen ? `${index * 50}ms` : '0ms'
                            }}
                        >
                            <div className="flex items-center justify-center text-center relative">
                                <span className="text-center">{option}</span>
                                {option === currentOption && (
                                    <div className="absolute right-0 w-2 h-2 bg-gradient-to-r from-purple-400 to-pink-400 rounded-full animate-pulse"></div>
                                )}
                            </div>
                        </div>
                    ))}
                </div>
            </div>
        </div>
    );
}