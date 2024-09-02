const IconCompactDown = (props: {
  size?: number;
  color?: string;
  onClick?: () => any;
}) => {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 24 24"
      fill={props.color || "currentColor"}
      width={props.size || 24}
      height={props.size || 24}
      onClick={props.onClick}
    >
      <path d="M3 18h18v-2H3zm0-5h18v-2H3zm0-7v2h18V6z"></path>
    </svg>
  );
};
export default IconCompactDown;
