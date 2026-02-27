import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';

import type { LucideIcon } from 'lucide-react';

interface CommonSettingsSelectorProps {
  items: string[];
  label?: string;
  placeholder?: string;
  onChange?: (value: string) => void;
  value?: string;
  disabled?: boolean;
  className?: string;
  icon?: LucideIcon;
}

export function CommonSettingsSelector({
  items,
  label,
  placeholder = 'Select an option',
  onChange,
  value,
  disabled = false,
  className = '',
  icon,
}: CommonSettingsSelectorProps) {
  const IconComponent = icon;

  return (
    <div className={`flex flex-col gap-3 w-full ${className}`}>
      {label && (
        <label className="text-sm font-semibold text-secondary tracking-wide transition-colors duration-200">
          {label}
        </label>
      )}
      <div className="relative w-full group">
        {/* Animated gradient background */}
        <div className="absolute inset-0 rounded-lg bg-gradient-to-r from-purple-500/20 via-blue-500/20 to-cyan-500/20 blur-xl opacity-0 group-hover:opacity-100 transition-opacity duration-500" />
        <div className="absolute inset-0 rounded-lg bg-gradient-to-r from-purple-500/10 via-blue-500/10 to-cyan-500/10 animate-pulse" />

        <Select value={value} onValueChange={onChange} disabled={disabled}>
          <SelectTrigger
            size="large"
            className="relative w-full backdrop-blur-md text-white bg-card/60 border-2 border-transparent bg-gradient-to-r from-purple-500/30 via-blue-500/30 to-cyan-500/30 bg-clip-padding hover:border-purple-500/50 transition-all duration-300 shadow-lg hover:shadow-xl hover:shadow-purple-500/20 [&_svg:not([class*='text-'])]:text-secondary"
          >
            <span className="flex min-w-0 items-center gap-2">
              {IconComponent && (
                <IconComponent className="w-4 h-4 text-white flex-shrink-0" />
              )}
              <SelectValue placeholder={placeholder} className="font-medium" />
            </span>
          </SelectTrigger>
          <SelectContent className="bg-popover border-none">
            {items.map((item) => (
              <SelectItem
                key={item}
                value={item}
                label={item}
                className="cursor-pointer hover:bg-gradient-to-r hover:from-purple-500/20 hover:via-blue-500/20 hover:to-cyan-500/20 focus:bg-gradient-to-r focus:from-purple-500/30 focus:via-blue-500/30 focus:to-cyan-500/30 transition-all duration-200 rounded-md"
              >
                {IconComponent && (
                  <IconComponent className="w-4 h-4 text-popover-foreground flex-shrink-0" />
                )}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </div>
    </div>
  );
}
