interface TextAvatarProps {
    text: string,
    color: { text: string, background: string },
    size: number,
    round: number,
}

const TextAvatar = ({text, color, size,round}: TextAvatarProps) => {
    return (
        <svg width={size} height={size} viewBox={`0 0 ${size} ${size}`} xmlns="http://www.w3.org/2000/svg">
            <rect width={size} height={size} fill={color.background} rx={round} ry={round}/>
            <text x="50%" y="50%" font-family="Arial" font-size={size * 0.5} text-anchor="middle" fill={color.text}>{text}</text>
        </svg>
    );
}
export default TextAvatar;