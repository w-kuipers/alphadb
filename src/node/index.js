"use strict";
const spawn = require("child_process").spawn;
const connect = ({ host, user, password, database, port = 3306 }) => {
    const pyprocess = spawn("python", [
        "wrapper.py",
        "connect",
        host,
        user,
        password,
        database,
        port
    ]);
    pyprocess.stdout.on("data", (data) => {
        console.log(data === null || data === void 0 ? void 0 : data.toString());
    });
};
connect({
    host: "localhost",
    user: "root",
    password: "test",
    database: "test"
});
