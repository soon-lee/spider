import {useEffect, useState} from 'react';
import {
    ApartmentOutlined,
    DeploymentUnitOutlined,
    GoldOutlined,
    MenuFoldOutlined,
    MenuUnfoldOutlined,
    RocketOutlined,
    UngroupOutlined
} from '@ant-design/icons';
import {Menu} from 'antd';
import {styled} from "styled-components";

const Wrapper = styled(Menu)`
    ul li {
        padding-left: 24px !important;
    }
`;

const CompactMenu = ({collapsed, onCollapse}: { collapsed: boolean, onCollapse: () => any }) => {

    const [items, setItems] = useState([
        {key: 'action', icon: <MenuUnfoldOutlined/>},
        {
            key: 'prepare',
            label: '爬虫准备',
            icon: <DeploymentUnitOutlined/>, children: [{
                key: 'website', icon: <GoldOutlined/>,
                label: '网站'
            },
                {
                    key: 'meta',
                    label: '元数据'
                },]
        },

        {
            key: 'construct',
            label: '爬虫构建',
            icon: <ApartmentOutlined/>,
            children: [{key: 'extractor', label: '提取器'},
                {key: 'workflow', label: '提取流'},
                {key: 'handler', label: '处理器'},],
        },
        {
            key: 'running',
            label: '爬虫执行',
            icon: <RocketOutlined/>,
            children: [{key: 'line', label: '流水线'},
                {key: 'instance', label: '运行实例'},],
        },
        {
            key: 'data',
            label: '数据管理',
            icon: <UngroupOutlined/>,
            children: [{key: 'visual', label: '数据可视'},
                {key: 'translation', label: '数据迁移'},],
        },
    ]);
    const handleCollapse = (param: any) => {
        if (param.key !== 'action') return;
        onCollapse();
    };

    useEffect(() => {
        setItems(prev => {
            return prev.map(item => {
                if (item.key === 'action') {
                    return {...item, icon: collapsed ? <MenuFoldOutlined/> : <MenuUnfoldOutlined/>}
                }
                return item;
            })
        });
    }, [collapsed]);

    return (
        <Wrapper
            defaultOpenKeys={['prepare', 'construct', 'running', 'data']}
            mode="inline"
            items={items}
            inlineCollapsed={collapsed}
            onClick={handleCollapse}
        />
    );
}
export default CompactMenu;