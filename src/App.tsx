import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Button from "./components/Button/Button";

import fileHandler from "./utils/FileHandler";
import ImageHolder from "./components/ImageHolder";



function App() {
    
    const [imagePath, setImagePath] = useState('');
    const [textImage, setTextImage] = useState('');

    
    function getCharImage(path: string) {
        let charImage: any;
        
        invoke("get_char_image", { path })
            .then((message) => {
                console.log(message);
                charImage = message;
            })
            .catch((e) => {
                console.error(e);
                charImage = e;
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
            <ImageHolder image={textImage}/>
        </div>
    );
}



export default App;
