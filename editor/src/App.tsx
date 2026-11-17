import { useState } from "react";

import { invoke } from "@tauri-apps/api/core";
import Container from "react-bootstrap/Container";
import Row from "react-bootstrap/Row";
import Col from "react-bootstrap/Col";
import Button from "react-bootstrap/Button";
import Form from "react-bootstrap/Form";

function App() {
  const [greetMsg, setGreetMsg] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <Container>
      <Row>
        <Col>
          <Form>
            <Form.Group className="mb-3" controlId="greet">
              <Form.Label>Greet</Form.Label>
            </Form.Group>

            <Button variant="primary"               type="submit"
                    onSubmit={(e) => {
                      e.preventDefault();
                      greet();
                    }}
            >Primary</Button>

          </Form>
          <p>{greetMsg}</p>
        </Col>
      </Row>
    </Container>
  );
}

export default App;
