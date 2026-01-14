import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';

interface TransparentCardProps {
  children: React.ReactNode;
  description?: string;
  title?: string;
  className?: string;
}

export function TransparentCard({ children, description = '', title = '', className = '' }: TransparentCardProps) {
  return (
    <Card className={`backdrop-blur-3xl border-none text-secondary ${className}`}>
      <CardHeader>
        <CardTitle>{title}</CardTitle>
        <CardDescription>{description}</CardDescription>
      </CardHeader>
      <CardContent className="flex flex-col gap-5">{children}</CardContent>
    </Card>
  );
}
