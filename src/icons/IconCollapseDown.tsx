const IconCompactDown = (props: {
  size?: number;
  color?: string;
  onClick?: () => any;
}) => {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 16 16"
      fill={props.color || "currentColor"}
      width={props.size || 24}
      height={props.size || 24}
      onClick={props.onClick}
    >
      <path d="M1.553 6.776a.5.5 0 0 1 .67-.223L8 9.44l5.776-2.888a.5.5 0 1 1 .448.894l-6 3a.5.5 0 0 1-.448 0l-6-3a.5.5 0 0 1-.223-.67z" />
    </svg>
  );
};
export default IconCompactDown;
