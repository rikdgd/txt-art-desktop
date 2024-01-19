import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Button from "./components/Button/Button";

import fileHandler from "./utils/FileHandler";



function App() {
    
    const [imagePath, setImagePath] = useState('');
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
    
    function selectImage() {
        fileHandler.selectFile()
            .then((val) => {
                if (val && !Array.isArray(val)) {
                    setImagePath(val);
                    
                } else {
                    console.log('Please select an image.');
                }
            })
    }
    
    return (
        <div className="container">
            <p>{imagePath}</p>
            <Button text='select image' OnClick={() => selectImage()}/>
            <Button text='convert image' OnClick={() => getCharImage(imagePath)}/>
        </div>
    );
}



export default App;
