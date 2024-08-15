import CompactMenu from '@components/CompactMenu';
import './App.css';
import Website from '@components/prepare/Website';
import { Layout, Breadcrumb } from 'antd';
import Sider from 'antd/es/layout/Sider';
import { HashRouter as Router, Route, Routes } from 'react-router-dom';
import { styled } from 'styled-components';
import { useState } from 'react';

const Wrapper = styled(Layout)`
  width: 100%;
  height: 100%;
`;
const Body = styled(Layout)`
  width: 100%;
  height: 100%;
  ul{
    height: 100%;
    padding: 0 5px;
  }
`;
const Main = styled(Layout)`
  width: 100%;
  height: 100%;
  padding: 5px;
  margin: 0;

  nav{
    padding: 5px;
  }

  main{
    overflow: hidden;
  }
`;
const App = () => {
  const [collapse, setCollapse] = useState(false);

  return (
    <Router>
      <Wrapper>
        <Layout.Header>
        </Layout.Header>
        <Body>
          <Sider trigger={null} collapsible collapsed={collapse}>
            <CompactMenu collapsed={collapse} onCollapse={() => setCollapse(prev => !prev)} />
          </Sider>
          <Main>
            <Breadcrumb>
              <Breadcrumb.Item>Home</Breadcrumb.Item>
              <Breadcrumb.Item>List</Breadcrumb.Item>
              <Breadcrumb.Item>App</Breadcrumb.Item>
            </Breadcrumb>
            <Layout.Content>
              <Routes>
                <Route path="/" element={"index"} />
                <Route path="/website" element={<Website />} />
              </Routes>
            </Layout.Content>
          </Main>
        </Body>
      </Wrapper>
    </Router>
  );
};

export default App;
