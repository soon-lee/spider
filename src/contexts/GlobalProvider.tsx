import {LocaleProvider} from "@contexts/LocaleProvider";
import {ThemeProvider} from "@contexts/ThemeProvider";

const GlobalProvider = (props:{children:any}) => {
  return (
    <ThemeProvider>
      <LocaleProvider>
        {props.children}
      </LocaleProvider>
    </ThemeProvider>
  );
};

export default GlobalProvider;