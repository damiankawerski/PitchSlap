import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';

interface TransparentCardProps {
  children: React.ReactNode;
  description?: string;
  title?: string;
  className?: string;
}

export function TransparentCard({ children, description = '', title = '', className = '' }: TransparentCardProps) {
  return (
    <Card className={`bg-card/60 backdrop-blur ${className}`}>
      <CardHeader>
        <CardTitle>{title}</CardTitle>
        <CardDescription>{description}</CardDescription>
      </CardHeader>
      <CardContent className="flex flex-col gap-5">{children}</CardContent>
    </Card>
  );
}
