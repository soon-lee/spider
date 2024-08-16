import { JSXElement } from 'solid-js';
import { styled } from 'solid-styled-components';

const Wrapper = styled.div`
  width: 100%;
  height: 100%;
  display: flex;
  flex-flow: column nowrap;
`;
const Header = styled.header<{ height?: string }>`
  height: ${props => props.height || '70px'};
  background-color: var(--color-container);
`;
const Body = styled.div`
  flex-grow: 1;
  display: flex;
  flex-flow: row nowrap;
`;
const Aside = styled.aside<{ width?: string }>`
  width: ${props => props.width || '200px'};
  background-color: var(--color-component);
`;
const Main = styled.main`
  flex-grow: 1;
  background-color: var(--color-background);
`;
const JuLayout = (props: { header?: JSXElement, aside?: JSXElement, main?: JSXElement, headerHeight?: string, asideWidth?: string }) => {
  return (<Wrapper>
    <Header height={props.headerHeight}>{props.header}</Header>
    <Body>
      <Aside width={props.asideWidth}>{props.aside}</Aside>
      <Main>{props.main}</Main>
    </Body>
  </Wrapper>);
}
export default JuLayout;