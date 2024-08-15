import {EditOutlined, EllipsisOutlined, SettingOutlined} from "@ant-design/icons";
import {Avatar, Card, Col, Descriptions, Row} from "antd";
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
    info: '一个用于前端开发者ertetgdg测试网站',
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
    info: '一个用于前端开发者测试网站tfytftiuhiuhoi789',
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
        <Row gutter={[16, 16]}>
            {
                websites.map((item, index) =>
                    <Col>
                        <Card hoverable style={{width: 300}}
                              actions={[
                                  <SettingOutlined key="setting"/>,
                                  <EditOutlined key="edit"/>,
                                  <EllipsisOutlined key="ellipsis"/>,
                              ]}
                              extra={<a href={item.address[0]} target="_blank">访问</a>}
                              title={item.title}
                        >
                            <Meta
                                avatar={<Avatar src={item.icon} shape={'square'}
                                                size={'large'}>{item.title[0]}</Avatar>}
                                description={item.info}

                            />
                            <Descriptions items={[
                                {
                                    label: '来源',
                                    children: item.origin.map((origin, index) => <a key={index} href={origin}
                                                                                    target="_blank">{origin}</a>)
                                },
                            ]}/>
                            <Descriptions items={[
                                {
                                    label: '地址',
                                    children: item.address.map((address, index) => <a key={index} href={address}
                                                                                      target="_blank">{address}</a>)
                                }
                            ]}/>
                        </Card>
                    </Col>)
            }
        </Row>
    );
}
export default Website;