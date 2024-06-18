---
title: docker和scp常用命令
---

tutorial: [link](https://docs.docker.com/language/python/build-images/)


创建虚拟环境
```cmd
cd /path/to/python-docker
python3 -m venv .venv
source .venv/bin/activate
(.venv) $ python3 -m pip install Flask
(.venv) $ python3 -m pip freeze > requirements.txt # 导出依赖
(.venv) $ touch app.py
```


docker文件demo
```dockerfile
# syntax=docker/dockerfile:1
FROM python:3.9.16-slim-bullseye
WORKDIR /app
COPY requirements.txt requirements.txt
RUN pip3 install -r requirements.txt
COPY . .
CMD [ "python3", "-m" , "flask", "run", "--host=0.0.0.0"]
```

build image
```cmd
docker build --tag python-docker .
```

启动docker
```cmd
docker run -d -p 8001:5000 python-docker
```

docker image搜索网址: [site](https://hub.docker.com/_/python)

上传文件夹到远程服务器
```
scp -r /path/to/local/source user@ssh.example.com:/path/to/remote/destination 
# 或者
scp -r /path/to/local/source user@ssh.example.com:.
```


