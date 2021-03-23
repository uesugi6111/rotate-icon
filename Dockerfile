FROM public.ecr.aws/lambda/provided:al2

ADD ./target/x86_64-unknown-linux-musl/release/rotate-icon ${LAMBDA_RUNTIME_DIR}/bootstrap
ADD ./.env ${LAMBDA_TASK_ROOT}/.env
ADD ./img/ ${LAMBDA_TASK_ROOT}/img/

CMD [ "lambda-handler" ]