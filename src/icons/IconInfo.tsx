const IconInfo = (props: {
  size?: number;
  color?: string;
  onClick?: () => any;
}) => {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="64 64 896 896"
      fill={props.color || "currentColor"}
      width={props.size || "24"}
      height={props.size || "24"}
      onClick={props.onClick}
    >
      <path d="M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448 448-200.6 448-448S759.4 64 512 64zm0 820c-205.4 0-372-166.6-372-372s166.6-372 372-372 372 166.6 372 372-166.6 372-372 372z"></path>
      <path d="M464 336a48 48 0 1096 0 48 48 0 10-96 0zm72 112h-48c-4.4 0-8 3.6-8 8v272c0 4.4 3.6 8 8 8h48c4.4 0 8-3.6 8-8V456c0-4.4-3.6-8-8-8z"></path>
    </svg>
  );
};
export default IconInfo;
