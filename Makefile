#DEV

build-server-dev:
	docker build -t tuah-server -f containers/images/Dockerfile . && docker build -t turn -f containers/images/turn/Dockerfile.turn .

run-server-dev:
	docker-compose -f containers/composes/dc.dev.yml up

clean-server-dev:
	docker-compose -f containers/composes/dc.dev.yml down

