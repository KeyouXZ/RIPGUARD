from ultralytics import YOLO

def main():
    # 1. Load model pretrained
    model = YOLO("yolov8n.pt") # local
    #model = YOLO("/content/dataset/yolov8n.pt") # colab

    # 2. Train
    model.train(
        data="data.yaml", # local
        #data="/content/dataset/data.yaml", # colab

        #epochs=50, #def
        epochs=200,
        imgsz=640,
        batch=16,
        device="cpu",
        workers=4,

        # augmentasi
        hsv_h=0.015,
        hsv_s=0.7,
        hsv_v=0.4,
        #degrees=0.0, #def
        degrees=5.0,
        translate=0.1,
        scale=0.5,
        fliplr=0.5,

        # logging (local)
        project="runs",
        name="rip_current_model",

        # logging (colab)
        #project="/content/dataset/runs",
        #name="rip_current_model",


        # biar nggak overwrite
        exist_ok=True,

        # Additional
        mosaic=1.0,
        mixup=0.1,
        copy_paste=0.1,
        patience=20,
    )

    # 3. Export ONNX
    model.export(format="onnx", opset=14, simplify=True)

if __name__ == "__main__":
    main()
