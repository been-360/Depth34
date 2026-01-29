import { useEffect } from "react";


export default function GetState() {
    useEffect(() => {
        const socket = new WebSocket("ws://localhost:3434");

        socket.onopen = () => {
            console.log("WebSocket connected")
        }

        socket.onmessage = (event) => {
            console.log(event)
        }
        
        return () => {
            socket.close()
        }
    }, [])

    return (
        <div>
            
        </div>
    )
}