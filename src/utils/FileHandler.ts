import { open } from "@tauri-apps/api/dialog";



class FileHandler {
    async selectFile() {
        const path =  await open({
            multiple: false,
            directory: false,
            filters: [{
                name: "Image",
                extensions: ['jpg', 'png']
            }],
        });
        
        return path;
    }
}



const fileHandler = new FileHandler();
export default fileHandler;
