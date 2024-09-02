const IconSave = (props: {
  size?: number;
  color?: string;
  onClick?: () => any;
}) => {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 1024 1024"
      fill={props.color || "currentColor"}
      width={props.size || 24}
      height={props.size || 24}
      onClick={props.onClick}
    >
      <path
        d="M704 128l192 192v512a64 64 0 0 1-64 64H192a64 64 0 0 1-64-64V192a64 64 0 0 1 64-64h512z m-64 64H384v96h256V192z m64 26.496V352H320V192H192v640h128V512h384v320h128V346.496l-128-128zM640 832V576H384v256h256z"
        fill="#000000"
        fill-opacity=".9"
        p-id="10515"
      ></path>
    </svg>
  );
};
export default IconSave;
