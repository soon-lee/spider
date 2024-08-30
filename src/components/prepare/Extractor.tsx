import {Button, List} from 'antd';
import Mapper from "@components/prepare/Mapper.tsx";
import {styled} from "styled-components";
import {PlusOutlined} from "@ant-design/icons";

const Wrapper = styled(List)`
    width: 100%;
    height: 80%;
    overflow: auto;
`;

const Extractor = () => {
    return (
        <Wrapper
            footer={<Button><PlusOutlined/></Button>}
            bordered
            dataSource={[1, 2, 3,]}
            renderItem={(item) => (
                <List.Item>
                    <Mapper/>
                </List.Item>
            )}
        />
    );
}
export default Extractor;