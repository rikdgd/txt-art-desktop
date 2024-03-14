import './OptionPanel.css';
import showIcon from '../../assets/icon-show.svg';
import hideIcon from '../../assets/icon-hide.svg';
import { useState } from 'react';



function OptionPanel({ children }: { children: React.ReactNode }) {
    
    const [isCollapsed, setIsCollapsed] = useState(false);
    
    if (isCollapsed) {
        return (
            <div className="OptionPanel">
                <img 
                    src={showIcon} 
                    alt="Show" 
                    onClick={() => setIsCollapsed(false)} 
                />
            </div>
        );
        
    } else {
        return (
            <div className="OptionPanel">
                <img 
                    src={hideIcon} 
                    alt="Hide" 
                    onClick={() => setIsCollapsed(true)} 
                    style={{marginBottom: '14px'}}
                />
                {children}
            </div>
        );
    }
}



export default OptionPanel;
