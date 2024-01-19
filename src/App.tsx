import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";



function App() {
    
    const [textImage, setTextImage] = useState('');
    
    function getCharImage(path: string): undefined {
        let charImage: any;
        
        invoke("getCharImage", { path })
            .then((data) => {
                charImage = data;
            })
            .catch((e) => {
                console.error(e);
            });
        
        setTextImage(charImage);
    }
    
    return (
        <div className="container">
            <button onClick={getCharImage('test')}>Convert to text</button>
            <br/>
            <p>{textImage}</p>
        </div>
    );
}



export default App;
