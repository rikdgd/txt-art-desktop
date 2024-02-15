import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Button from "./components/Button/Button";

import { clipboard } from "@tauri-apps/api";

import fileHandler from "./utils/FileHandler";
import ImageHolder from "./components/ImageHolder";
import OptionPanel from "./components/OptionPanel";



function App() {
    
    const [imagePath, setImagePath] = useState('');
    const [textImage, setTextImage] = useState<any>('');
    
    
    function getCharImage(path: string) {
        invoke("get_char_image", { path })
            .then((message) => {
                setTextImage(message);
            })
            .catch((e) => {
                console.error(e);
                setTextImage(e);
            });
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

    async function copyToClipboard() {
        await clipboard.writeText(textImage);
    }
    
    
    
    return (
        <div>
            <div>
                <OptionPanel>
                    <Button text='select image' OnClick={() => selectImage()}/>
                    <Button text='convert image' OnClick={() => getCharImage(imagePath)}/>
                    <Button text='copy to clipboard' OnClick={async () => copyToClipboard()}/>
                </OptionPanel>
            </div>
            
            <div className="container">
                <ImageHolder image={textImage}/>
            </div>
        </div>
    );
}



export default App;
