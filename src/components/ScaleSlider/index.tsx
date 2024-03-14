import { useState } from "react";



interface ScaleSliderProps {
    max: number;
    min: number;
    onChange: (e: number) => void;
}

/**
 * Function component that renders a slider input element that can be scaled within a specific range.
 *
 * @param {number} max - the maximum value the slider can have
 * @param {number} min - the minimum value the slider can have
 * @return {JSX.Element} the slider input element
 */
function ScaleSlider(sliderProps: ScaleSliderProps) {
    
    const [sliderValue, setSliderValue] = useState(1.0);
    
    const onChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const inputValue = parseFloat(e.target.value);
        setSliderValue(inputValue);
        sliderProps.onChange(inputValue);
    }
    
    return(
        <div>
            <input 
                type="range" 
                max={sliderProps.max} 
                min={sliderProps.min} 
                value={sliderValue} 
                onChange={onChange}
            />
        </div>
    );
}



export default ScaleSlider;
