import './Button.css';


function Button(
    {
        text, 
        OnClick
    }: {
        text: string, 
        OnClick: () => void
    }) {
        
    return ( 
        <div>
            <button className='MainButton' onClick={OnClick}>{text}</button>
        </div>
     );
}

export default Button;
