import { createContext, createEffect, createResource, createSignal, useContext } from "solid-js";
import zh_CN from '@assets/locales/zh-CN.json';
import en_US from '@assets/locales/en-US.json';

export const LocaleContext = createContext<{
  getTranslation: (key: string) => string;
  setLocale: (locale: string) => void;
  locale: () => string;
}>({
  getTranslation: (key: string) => "",
  setLocale: (locale: string) => { },
  locale: () => "",
});
export const LocaleProvider = (props: { children: any }) => {

  const [locale, setLocale] = createSignal(localStorage.getItem("nsp-ux-locale") || 'zh-CN');

  const fetchTranslations = (locale: string) => {
    return locale === 'zh-CN' ? zh_CN as { [key: string]: string } : en_US as { [key: string]: string };
  };

  const [translations] = createResource(locale, fetchTranslations);
  const getTranslation = (key: string) => {
    const translation = translations();
    return translation ? translation[key] : key;
  };

  createEffect(() => {
    document.documentElement.setAttribute('lang', locale());
    localStorage.setItem("nsp-ux-locale", locale());
  });

  return (
    <LocaleContext.Provider value={{ getTranslation, setLocale, locale }}>
      {props.children}
    </LocaleContext.Provider>
  );
};
export const useLocale = () => useContext(LocaleContext);