const spawn = require("child_process").spawn;

interface ConnectProps {
  host: string;
  user: string;
  password: string;
  database: string;
  port?: number;
}

const connect = ({ host, user, password, database, port = 3306 }: ConnectProps) => {
  const pyprocess = spawn("python", [
    "wrapper.py",
    "connect",
    host,
    user,
    password,
    database,
    port
  ]);

  pyprocess.stdout.on("data", (data: unknown) => {
    console.log(data?.toString());
  });
};

connect({
  host: "localhost",
  user: "root",
  password: "test",
  database: "test"
});
