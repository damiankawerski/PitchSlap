import React from 'react';
import { TransparentCard } from '@/components/ui/transparent-card';
import { Button } from '@/components/ui/button';
import { cn } from '@/lib/utils';
import { Plus, Trash2, Settings2 } from 'lucide-react';

export interface EffectCardProps {
  title: string;
  description: string;
  image: React.ReactNode;
  className?: string;
  active?: boolean;
  onToggle?: () => void;
  onOptions?: () => void;
}

export function EffectCard({
  title,
  description,
  image,
  className,
  active,
  onToggle,
  onOptions,
}: EffectCardProps) {
  return (
    <TransparentCard
      title={title}
      description={description}
      className={cn(
        'group relative overflow-hidden transition-all duration-300 flex flex-col h-full w-[300px]',
        active
          ? 'shadow-[0_0_20px_-5px_hsl(var(--primary)/0.3)] ring-1 ring-primary/30'
          : 'hover:bg-accent/5',
        className,
      )}
    >
      {active && (
        <div className="absolute top-2 right-2 bg-primary/90 text-primary-foreground text-xs font-bold px-2 py-1 rounded-full shadow-lg backdrop-blur-md flex items-center gap-1.5 animate-in fade-in zoom-in duration-300">
          <span className="relative flex h-2 w-2">
            <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-white opacity-75"></span>
            <span className="relative inline-flex rounded-full h-2 w-2 bg-white"></span>
          </span>
          Active
        </div>
      )}

      <div className="relative aspect-[16/9] w-full overflow-hidden rounded-md border border-border/50">
        <div className="absolute">{image}</div>
      </div>

      {/* Actions */}
      <div className="grid grid-cols-2 gap-2 mt-auto pt-2 w-full">
        <Button
          variant={active ? 'destructive' : 'default'}
          size="sm"
          className="w-full gap-2 cursor-pointer min-w-0 px-2"
          onClick={onToggle}
        >
          {active ? <Trash2 className="h-4 w-4 shrink-0" /> : <Plus className="h-4 w-4 shrink-0" />}
          <span className="truncate">{active ? 'Remove' : 'Add'}</span>
        </Button>
        <Button
          variant="secondary"
          size="sm"
          className="w-full gap-2 cursor-pointer min-w-0 px-2"
          onClick={onOptions}
        >
          <Settings2 className="h-4 w-4 shrink-0" />
          <span className="truncate">Options</span>
        </Button>
      </div>
    </TransparentCard>
  );
}
