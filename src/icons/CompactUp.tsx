const CompactUp = (props: { size?: number, color?: string, onClick?: () => any }) => {
    return (
        <svg xmlns="http://www.w3.org/2000/svg"
             viewBox="0 0 24 24"
             fill={props.color || "currentColor"}
             width={props.size || 24}
             height={props.size || 24}
             onClick={props.onClick}>
            <path d="M3 18h13v-2H3zm0-5h10v-2H3zm0-7v2h13V6zm18 9.59L17.42 12 21 8.41 19.59 7l-5 5 5 5z"></path>
        </svg>
    );
}
export default CompactUp;