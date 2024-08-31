const TextIcon = (props: {
  text: string;
  width?: number;
  height?: number;
  type?: "circle" | "square" | "round";
  color?: string;
  onClick?: () => any;
}) => {
  const width = props.width || 24;
  const height = props.height || 24;
  const fontSize = Math.round(Math.min(width, height) * 0.65);
  const handleText = () => {
    if (props.text.length > 0) {
      return props.text.length > 2 ? props.text.substring(0, 1) : props.text;
    }
    return String.fromCharCode(Math.random() * 26 + 65);
  };
  const handleRadius = () => {
    switch (props.type) {
      case "circle":
        return Math.min(width, height) / 2;
      case "square":
        return 0;
      case "round":
        return 5;
      default:
        return 5;
    }
  };
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox={`0 0 ${width} ${height}`}
      width={width}
      height={height}
      onClick={props.onClick}
    >
      <rect
        fill={props.color || "currentColor"}
        width={width}
        height={height}
        rx={handleRadius()}
        ry={handleRadius()}
      />
      <text
        x={`${width / 2}`}
        y={`${height / 2}`}
        text-anchor="middle"
        font-size={`${fontSize}px`}
        dominant-baseline="middle"
      >
        {handleText()}
      </text>
    </svg>
  );
};
export default TextIcon;
