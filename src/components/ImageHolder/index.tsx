import { useState, useEffect } from 'react';
import './ImageHolder.css';


function ImageHolder({ image }: { image: string | null}) {

    const [textImage, setTextImage] = useState('');
    const [fontSize, setFontSize] = useState(12);
    const testImage = `
    ...........................................::HXXXXXXXXHX+:+++HXXH:.......................::+++++++++
    ..........................................::HX####XXXHHH++++HHX##X+.........................:::+++++
    ..........................................:H#####X#XXXXHH+HHXX####+:............................:+++
    .........................................:+X###XXX#XXXXXXXX#######XH+::......::::::::::..........:++
    .........................................:X####X##XXHXXX#############XHH++::+HXXXXHHHHHH+++::::::::+
    .:::::::.................................+X###XX###XXXXX##################XXX#########XH+HXH+++HH+++
    ++++++++++++++::::::::..................:+X###XX######X###############################H++HH+++++HHH+
    XX#XXXXXXXXXXXXXHHHHH+++++++++++:::::::::+X##XXX##XXXXX#################XXX###########XXXXH+::::+HHH
    ###################XXXXXXXXXXHHHHHHHHHH++HXXXXXXXXXXXXXX##################X###########XXHH+++:::++HH
    ############################XHHHHHXXXXXXXXXXXXXXXHHHXXXXX##############################XXXXXH+++++HH
    ############################XHHHHHXXXHXXXXXHHHHHHHHHHXXXXX################################XXXHH+HHHX
    ############################XHHHXXXXHHHHXXXHHHHHHHHHHHHHHHXX############################XXXXXXXXXXX#
    #############################XHXXXXHH+++++HHHHHHHHHHHHHHHHHXXXHX###########################XXXXX####
    ###########################XXHHHHHHH+::::::+HHHHHHH+++++HHHXXXHHXXXXXX####################XXXX######
    ######################XXXXH+::::::::::.::+::+HHHHHHH++:+++++HHH+HXXHXXX##################XXX########
    ###############XXXXHHH+++:::::::::::++::++++++++HHHHH++H+++++++HHXHHHXX##XX############XXXX#########
    #############XXH++:::::::..:::::::::+++:+++++++:+++HHH+++::+++++HHHH+HX#XXX#########XXHHXX##########
    ######XXXXXXXXXHH++:::.......::::::::::::++++++::++HHH+:+H+:+++++HHHHXXXXXX########XXHXXX##########X
    #####XXXXXXXXXXXXH+::.......::::::++:.:++::+++::::++HHH+++++++++++HXXXXXXXX########XXXX##########XXX
    #####XXXXXXXXXXXXH+:.......::::::::::::++:...:::::::::+++:::++++++XXXXXXXXX#######XXXX##########XXXX
    XXXXXXXXXXXXXXXXH+:.......:::::::::::::+++:......:::::::+++++++++HXXXXXXXXX#####XXXXX#########XXXXXX
    XXXXXXXXXXXHHHHH+:......::::::+++::::+++++:::::..:::+:+++++HH++++HXXXXXXXXXXXXXXXXXX#########XXXXXXX
    XXXXXHHHHHHHHHH+:::::::::::::++++++++++++++:::+H+::::+HXXH+HHHHHHHHXXXXXXXXXXXXXXXX######XXXXXXXXXXX
    XXXHHHHH+++++H+::::::::::::+++++++HHHHHHHH++++HXXXH::HX##XXXXXXXHHHHXXXXXXXXXXXXXX####XXXXXXXXXXXXXX
    HHH+++++:::::::...::::::++++++HHHHHHHHHXHHHHHHXXXXH+:+XX#####XXXHHHHHHHXXXXXXXHHX###XXXXXXXXXXXXXXXX
    +::::::............::::::++++++HHHHHHHHHHHHHXXXHHH++++HXX####XH++++++++HHXHH+++HXXXXXXXXXXXXXXXXXXXX
    :::::....................::::+++++HHHHHHHHHHHHH+HHHHXXXX####XXH++++++:::+++++++HXXXXXXXXXXXXXXXXXXXX
    ..........................:::::::::+++HHHHHXHHHXXXXXXX#####XXH++++:::..:::++HHH+HHXXXXXXXXXXXXHXXX##
    .......................:::::::::::::::::+++HHHXXXXXXX####XXHH++++:::...:.:+++++HHXXXXXXXXXXHHHHXX###
    .....................::..................::::++HHHHXXXXXXHHH++:::::..:::::::::+HXXXXXXXXXXHHHXXX####
    .....................:::::::::::::::.:::.......:::+++HH++++:::::::.::::::::..+HHHHHHHXXXXXXXXXX##XXX
    ......................::::++++++++++::::::::::....:::::+++:::::::::::::::::.:+HHHHHHHXXXXXXXX#XXXXXX
    ...............:.::::::::::::::::::::::::::::::::::::::+++++::::::::::::+::.:+HHHHHHHXXXXXXXXXXXXXXX
    ..............::::::::::::::::::::::::::::::::::::::::::++++:::::::::++++::.:+HHHHHHHXXXXXXXXXXXXXXX
    ..............::::::::::::::::::++::::::::::..:::::::::+++::::+++++++++++::::++HHHHHXXXXXXXXXXXXXXXX
    ..............::::::::::::::::::::::::::........::::::::::::::+::+++++++++::::+HHHHXXXXXXXXXXXXXXXXX
    .............::::::::..::::::+:::::::................:::::::::....::::+++:::::+HHHXXXXXXXXXXXXXXXXXX
    ... .............::::......:::::::::................::::::::..:::::::::+:::.:++HHXXXXXXXXXXXXXXXXXXX
         ..............:::::::::.:::::::::...... ........:::::::::::::::::::::..:+HHHXXHHHHHHHXXXXXXXXXX
          ..........::::::.........::...........   ........:::::::::::::++:::..:+HHHHHHHHHHHHXXXXXXXXXXX
          ...........::::::::::::::::.............  ........:::::::::.:::::::..:+HHHH++HHHHXXXXXXXXXXXXX
          ................:::++::::.................. .......:::::::.:::::::..:+HHH+++HHHXXXXXXXXXXXXXXX
          .................:::::::::........................::::::::::::::....:+++++++HHXXXXXXXXXXXXXXXX
          ............................:.................::::::::::.::::::...:::::+++HHXXXXXXXXXXXXXXXXX#
       .  .........:...   ...        ...................:::.:::::::::::::::::::::++HXXXXXXXXXXXXXXXXX###
      ..    ...................       ...................::..:::::+:::::::++:::++HXXXXXXXXXXXXXXXXXX##XX
      ..     ............::::......  .......................::::::::::::::::+++HHHXXXXXXXXXXXXXXXXX##XHX
    . ..     ...........:::::::............:................::::::::::::::::+HHHHHXXXXXXXXXXXXXXXXX##XXX
    ........... .............:::..........::..   ...........:::::..::::::++++HHHHHXXXXXXXXXXXXXXXXXXX###
    :...............................:.....:.... ...............::....:::++++++HHHHXXXXXXXXXXXXXXXXXXX###
    `;
    
    useEffect(() => {
        if(image) {
            setTextImage(image);
        } else {
            setTextImage(testImage);
        }
    }, [image]);


    function onSliderChange(event: any) {
        setFontSize(event.target.valueAsNumber);
    }
    
    
    
    return(
        <div>
            <div>
                <input type="range" min={1} max={100} value={fontSize} onChange={onSliderChange}/>
            </div>

            <p className="ImageContainer" style={{ fontSize: fontSize }}>
                {textImage}
            </p>
        </div>
    );
}



export default ImageHolder;
