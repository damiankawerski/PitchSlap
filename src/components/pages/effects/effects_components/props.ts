export interface EffectProps {
    title: string;
    onChangeHandler: (name: string) => void;
    isActive: boolean;
    slug: string;
}