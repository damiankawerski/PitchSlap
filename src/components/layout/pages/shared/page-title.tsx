import type {LucideIcon} from 'lucide-react'

interface PageTitleProps {
  title: string;
  icon?: LucideIcon;
}

export function PageTitle({ title, icon: Icon }: PageTitleProps) {
  return (
    <h1 className="text-2xl font-bold text-secondary tracking-wide">
      {Icon && <Icon className="inline-block mr-2" />}
      {title}
    </h1>
  );
}