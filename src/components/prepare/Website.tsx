import { SettingOutlined, EditOutlined, EllipsisOutlined } from "@ant-design/icons";
import { Card, Avatar, Row, Col } from "antd";
import Meta from "antd/es/card/Meta";

const websites = [{
  title: '测试网站',
  info: '一个用于前端开发者测试网站',
  icon: null,
  origin: [''],
  address: ['']
}, {
  title: '测试网站',
  info: '一个用于前端开发者测试网站',
  icon: null,
  origin: [''],
  address: ['']
}, {
  title: '测试网站',
  info: '一个用于前端开发者测试网站',
  icon: null,
  origin: [''],
  address: ['']
}, {
  title: '测试网站',
  info: '一个用于前端开发者测试网站',
  icon: null,
  origin: [''],
  address: ['']
}, {
  title: '测试网站',
  info: '一个用于前端开发者测试网站',
  icon: null,
  origin: [''],
  address: ['']
}, {
  title: '测试网站',
  info: '一个用于前端开发者测试网站',
  icon: null,
  origin: [''],
  address: ['']
}, {
  title: '测试网站',
  info: '一个用于前端开发者测试网站',
  icon: null,
  origin: [''],
  address: ['']
}, {
  title: '测试网站',
  info: '一个用于前端开发者测试网站',
  icon: null,
  origin: [''],
  address: ['']
}, {
  title: '测试网站',
  info: '一个用于前端开发者测试网站',
  icon: null,
  origin: [''],
  address: ['']
}, {
  title: '测试网站',
  info: '一个用于前端开发者测试网站',
  icon: null,
  origin: [''],
  address: ['']
}, {
  title: '测试网站',
  info: '一个用于前端开发者测试网站',
  icon: null,
  origin: [''],
  address: ['']
}]
const Website = () => {
  return (
    <Row>
      {
        websites.map((item, index) =>
          <Col>
            <Card style={{ width: 300 }}
              actions={[
                <SettingOutlined key="setting" />,
                <EditOutlined key="edit" />,
                <EllipsisOutlined key="ellipsis" />,
              ]}
            >
              <Meta
                avatar={item.icon ? item.icon : <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100">
                  <rect x="0" y="0" width="100" height="100" fill="#FF5722" stroke="#000" stroke-width="2" />
                  <text x="50" y="60" font-family="Arial" font-size="40" text-anchor="middle" fill="#FFFFFF">A</text>
                </svg>}
                title={item.title}
                description={item.info}
              /> </Card>
          </Col>)
      }
    </Row>
  );
}
export default Website;