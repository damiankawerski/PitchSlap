import { 
  Select, 
  SelectContent, 
  SelectItem, 
  SelectTrigger, 
  SelectValue 
} from "@/components/ui/select";

interface CommonSettingsSelectorProps {
  items: string[];
  label?: string;
  placeholder?: string;
  onChange?: (value: string) => void;
  value?: string;
  disabled?: boolean;
  className?: string;
}

export function CommonSettingsSelector({
  items,
  label,
  placeholder = "Select an option",
  onChange,
  value,
  disabled = false,
  className = "",
}: CommonSettingsSelectorProps) {
  return (
    <div className={`flex flex-col gap-3 w-full ${className}`}>
      {label && (
        <label className="text-sm font-semibold text-foreground/90 tracking-wide transition-colors duration-200">
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
            className="relative w-full backdrop-blur-md bg-card/60 border-2 border-transparent bg-gradient-to-r from-purple-500/30 via-blue-500/30 to-cyan-500/30 bg-clip-padding hover:border-purple-500/50 transition-all duration-300 shadow-lg hover:shadow-xl hover:shadow-purple-500/20"
          >
            <SelectValue 
              placeholder={placeholder}
              className="font-medium"
            />
          </SelectTrigger>
          <SelectContent 
            className="backdrop-blur-2xl bg-popover/95 border-2 border-purple-500/30 shadow-2xl shadow-purple-500/10 animate-in fade-in-0 zoom-in-95 duration-200 w-full"
          >
            {items.map((item) => (
              <SelectItem
                key={item}
                value={item}
                className="cursor-pointer hover:bg-gradient-to-r hover:from-purple-500/20 hover:via-blue-500/20 hover:to-cyan-500/20 focus:bg-gradient-to-r focus:from-purple-500/30 focus:via-blue-500/30 focus:to-cyan-500/30 transition-all duration-200 rounded-md mx-1 my-0.5"
              >
                <span className="flex items-center gap-2.5">
                  <span className="w-2 h-2 rounded-full bg-gradient-to-r from-purple-500 via-blue-500 to-cyan-500 shadow-sm shadow-purple-500/50" />
                  <span className="font-medium">{item}</span>
                </span>
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </div>
    </div>
  );
}
