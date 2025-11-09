import {EffectProps} from './props';
import EffectCard from './EffectCard';
import DemonImage from '/src/assets/demon.jpg'; 

export default function TestingVoiceEffect(props: EffectProps) {
    return (
        <div>
            <EffectCard
                title={props.title}
                onChangeHandler={props.onChangeHandler}
                isActive={props.isActive}
                image={DemonImage}
                slug={props.slug}
            />
        </div>
    );
}
